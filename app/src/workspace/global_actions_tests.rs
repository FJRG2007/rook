use std::fs;
use std::path::{Path, PathBuf};

/// Every `.rs` file under the crate, so a new call site cannot be added in a
/// directory the check does not know about.
fn sources() -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![Path::new(env!("CARGO_MANIFEST_DIR")).join("src")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                files.push(path);
            }
        }
    }
    files
}

/// `dispatch_global_action` takes its argument by value and downcasts it to the
/// type the handler was registered with, so an action registered for `()` has to
/// be dispatched with `()`. Passing `&()` makes the argument a `&()`, the
/// downcast fails, and the dispatch is dropped having done nothing but log.
///
/// Twenty-eight call sites did that, `workspace:save_app` among them, which is
/// why the session was only ever written by the few paths that got it right,
/// and why tabs did not come back after a shutdown. Nothing about it fails to
/// compile, so it is checked here instead.
#[test]
fn no_global_action_is_dispatched_with_a_reference_to_unit() {
    let mut offenders = Vec::new();
    for path in sources() {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for (index, line) in text.lines().enumerate() {
            let dispatches = line.contains("dispatch_global_action(");
            if dispatches && line.contains(", &())") {
                offenders.push(format!("{}:{}", path.display(), index + 1));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "these pass &() and are dropped by the downcast; pass (): {}",
        offenders.join(", ")
    );
}

/// A global action's name is matched against the registry verbatim. Nothing
/// reports a name that is registered nowhere - the dispatch is dropped - and
/// `root_view::open_new` shipped carrying a second colon it should not have
/// had. Every registered name has exactly one.
#[test]
fn no_global_action_name_carries_a_double_colon() {
    let mut offenders = Vec::new();
    for path in sources() {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for (index, line) in text.lines().enumerate() {
            let Some((_, rest)) = line.split_once("dispatch_global_action(\"") else {
                continue;
            };
            let Some((name, _)) = rest.split_once('"') else {
                continue;
            };
            if name.contains("::") {
                offenders.push(format!("{}:{} {}", path.display(), index + 1, name));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "these action names carry a double colon and match nothing: {}",
        offenders.join(", ")
    );
}
