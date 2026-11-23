use super::*;

#[test]
fn per_session_vars_match_case_insensitively() {
    assert!(is_per_session_var(&OsString::from(
        TERMINAL_SESSION_UUID_ENV
    )));
    assert!(is_per_session_var(&OsString::from(
        FOCUS_URL_ENV.to_ascii_lowercase()
    )));
    assert!(!is_per_session_var(&OsString::from("PATH")));
}
