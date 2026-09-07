# The session is not saved on shutdown

**Symptom.** Restore session is on, but after shutting the machine down and starting it again the tabs do not come back. Usually a couple return, often stale, sometimes none, and their names and order are gone.

## Cause

Nothing wrote the session when the app was closing, and almost nothing wrote it while it was open.

`save_app` in `app/src/workspace/global_actions.rs` is the only producer of `ModelEvent::Snapshot`. It was reached from exactly four window callbacks - move, resize, focus change, close - plus a handful of scattered dispatches from menu and agent code. Two consequences:

- **Opening, closing, renaming and reordering tabs write nothing.** None of those touch a window callback, so tab structure was only ever persisted if the user happened to move, resize or refocus a window afterwards.
- **Shutdown wrote nothing at all.** `on_window_will_close` returns early once the stage is `Terminating`, and `on_will_terminate` stopped the persistence writer without taking a snapshot first.

So what came back was the layout as of the last window move or focus change, which can be arbitrarily far behind. That is exactly "a couple of tabs, stale, or none".

## Fix

Two changes, because there are two ways to lose a session.

**A graceful exit** - which is what a normal Windows shutdown does - now snapshots before the writer stops. Global actions run their handlers inline (`dispatch_global_action_internal` calls them directly, then flushes effects), so the snapshot reaches the writer's channel ahead of the event that stops it, and `terminate` joins the thread rather than abandoning it.

**An abrupt stop** - the wall switch, a power cut, a killed process - never reaches that path, so the session is also written every 15 seconds. That interval is the ceiling on how much of a session an unexpected stop can take.

To keep a timer affordable, a snapshot equal to the last one written is dropped before it reaches the writer. An idle app builds a snapshot every 15 seconds and stops there, instead of re-running a delete-and-insert over every window, tab and pane. Window drags benefit from the same check for a different reason: they fire this path on every frame.

`app/src/lib.rs`, `app/src/persistence/mod.rs`, `app/src/workspace/global_actions.rs`.

## Durability

Writes that complete survive a power cut. The connection runs in WAL with SQLite's default `synchronous`, so each commit is flushed. The problem was never that writes were lost, only that they were not made.

The database on the machine this was diagnosed on showed repeated `A WAL mode database file was recovered` warnings, which is how it was established that ungraceful exits actually happen here rather than being hypothetical.

## What this does not cover

Anything changed in the last 15 seconds before an abrupt stop is still lost. Making that window smaller means writing more often; 15 seconds was chosen as the point where the cost is invisible and the loss is a few seconds of tab arrangement.
