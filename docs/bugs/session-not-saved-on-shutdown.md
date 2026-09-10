# The session is not saved on shutdown

**Symptom.** Restore session is on, but after shutting the machine down and
starting it again the tabs do not come back. Usually a couple return, often
stale, sometimes none, and their names and order are gone.

This was diagnosed twice. The first pass found a real gap and fixed it with a
timer; the timer then ran for two days without writing anything, because of a
second defect that hid it. Both are recorded here, because the second one is
the interesting one and the first one alone reads as a complete explanation.

## Cause 1: nothing wrote the session while it was closing

`save_app` in `app/src/workspace/global_actions.rs` is the only producer of
`ModelEvent::Snapshot`. It was reached from four window callbacks - move,
resize, focus change, close - plus scattered dispatches from menu and agent
code. `on_window_will_close` returns early once the stage is `Terminating`,
and `on_will_terminate` stopped the persistence writer without snapshotting
first, so what came back was the layout as of the last window move or focus
change, arbitrarily far behind.

The fix was a snapshot on the way out and a write every 15 seconds, the
latter bounding what an abrupt stop can take.

## Cause 2: the timer's dispatch was dropped before it reached the handler

The timer ran. Its saves did nothing.

There are two `dispatch_global_action` methods, and they want opposite
arguments for the same action:

```rust
// AppContext: takes the argument by value, then boxes it for the downcast.
pub fn dispatch_global_action<T: 'static + Any>(&self, name: &str, arg: T)

// MutableAppContext: takes it already erased.
pub fn dispatch_global_action(&mut self, name: &str, arg: &dyn Any)
```

An action registered for `()` must be given `()` through the first and `&()`
through the second. Passing `&()` to the first makes `T = &()`, the downcast
to `()` fails, and the dispatch is discarded.

Every call site in the tree is on the second method and correct, except one:
the autosave, which is on the first and passed `&()`. Finding that took two
wrong sweeps. The first changed all of them and the compiler rejected the
fifteen it could see; the second missed thirteen more in `app_menus.rs`,
which is `#[cfg(target_os = "macos")]` and so is not compiled by a Linux or
Windows job at all. Only the macOS runner reports on that file.

Nothing about that fails to compile - both forms are valid `T` - and in a
release build nothing is visible either. `add_global_action` carries a
`debug_assert!` on the failed downcast and a `report_error!`, and in a build
with assertions off and no telemetry host the only trace is one line in the
log file:

```
[ERROR] Could not downcast argument for action [action=workspace:save_app]
```

### How it was established

Counted in the user's own logs, not reasoned about:

| Log | save_app dispatched | dropped by the downcast |
| --- | --- | --- |
| The session open across the shutdown | 35 | 23 |
| A longer session the next day | 313 | 92 |

The dropped ones are exactly 15 seconds apart - 04:00:01, 04:00:16, 04:00:31,
04:00:46, 04:01:01 - which identifies them as the timer rather than anything
a user did. The window-event saves, which pass `&()` through the reference
method, are in the other column and had been working all along. That is why
the loss looked arbitrary: some tabs survived, on the schedule of whenever a
window had last been moved or focused.

## Cause 3: shutdown persists the app taking itself apart

Closing the last window tears the workspace down a tab at a time, and every
one of those closes dispatches `save_app` again. From the same log:

```
04:01:03Z storing data for closed tab
04:01:03Z dispatching global action for workspace:save_app
04:01:03Z dispatching global action for workspace:save_app
04:01:03Z dispatching typed action: PaneGroupAction::HandleFocusChange
04:01:03Z dispatching global action for workspace:save_app
04:01:03Z No windows left, terminating app
04:01:03Z application will terminate
```

Those saves are on the reference method, so they were not dropped. Each one
records a workspace with one fewer tab than the last, and the final state on
disk is whatever the teardown had reached. `save_app_state` deletes before it
inserts, so a snapshot with no windows at all does not fail - it leaves
nothing to restore.

## Cause 4: the OS kills the shells first, and each dead shell closed its tab

This is the one that survives every fix above, and the reason the symptom was
"exactly one tab comes back" rather than "a few". Found on the build that
already carried causes 1 to 3 fixed, after a Windows Update restart:

```
03:20:24Z Block finished with new state DoneWithNoExecution   (x5)
03:20:24Z storing data for closed tab                         (x5)
03:20:36Z dispatching global action for workspace:save_app    (every 15s)
...
03:21:59Z No windows left, terminating app
```

Five shells ended in the same second, at an idle prompt, with no key pressed.
Ending a session, Windows kills the console processes it can reach - every
shell inside a Rook tab - and gets to the window itself much later: 95
seconds here. `terminal_pane.rs` closed a pane whenever its shell exited, so
each tab was removed as its shell died. The autosave then did its job and
wrote, every 15 seconds, a session with one tab in it.

Nothing was lost in the write. The workspace had genuinely been reduced to
one tab by the time anything was saved, which is why the cure was never
going to be another save path.

The exit reason does not tell the two apart - the local event loop reports
`ShellProcessExited` whether the shell was killed or told to `exit` - and the
exit code is not carried as far as the pane. What the pane can see is the
command the shell was running when it went: an idle prompt, in every one of
these. A pane now closes on a shell exit only when that exit was asked for -
`exit` or `logout` at the prompt, or Rook shutting the pty down itself - and
otherwise stays under the "process terminated" banner it was already shown,
the same treatment a shell that dies before bootstrapping has always had.
Kept, the tab is in the snapshot and restores with a fresh shell in its
directory.

`app/src/terminal/view.rs` (`shell_exit_was_requested`),
`app/src/pane_group/pane/terminal_pane.rs`.

## Fix

- The autosave dispatches `()` through the by-value method.
- `save_app` stops writing once the stage is `Terminating`, and refuses a
  snapshot with no windows.
- The session is captured where it is still whole: on entry to
  `on_should_terminate_app`, which is where a logout, a restart or an OS
  shutdown arrives, and immediately before the last window closes. The
  snapshot in `on_will_terminate` is gone; by the time it ran there were no
  windows left to record.

`app/src/lib.rs`, `app/src/persistence/mod.rs`,
`app/src/workspace/global_actions.rs`.

## Durability

Writes that complete survive a power cut. The connection runs in WAL with
SQLite's default `synchronous`, so each commit is flushed. The problem was
never that writes were lost, only that they were not made.

The database showed repeated `A WAL mode database file was recovered`
warnings, which is how it was established that ungraceful exits actually
happen here rather than being hypothetical.

## What this does not cover

Anything changed in the last 15 seconds before an abrupt stop is still lost.

A shell that exits on its own for any other reason - it crashes, or a
program ends it - now leaves its tab open with the banner rather than
closing it. That is deliberate: there is no way to tell that apart from the
OS killing it, and a tab left open is one keystroke to close, where a tab
closed on a shutdown was gone.

A text scan cannot tell the two dispatch methods apart, so there is no test
for the argument shape; the `debug_assert!` in `add_global_action` is the
guard, and it only fires where assertions are on. A dispatch whose action
name matches nothing registered is checked, in
`app/src/workspace/global_actions_tests.rs` - `root_view::open_new` in the
URI handler had a second colon and reached no handler at all.
