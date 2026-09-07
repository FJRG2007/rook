use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Sentinel title that identifies structured CLI-agent events sent via OSC 777.
pub const CLI_AGENT_NOTIFICATION_SENTINEL: &str = "rook://cli-agent";

/// The sentinel the published CLI-agent plugins send.
///
/// Those plugins - claude-code-rook, codex-rook, gemini-cli-rook - live in their own
/// repositories, were built against upstream, and are already installed on users'
/// machines. Their half of this protocol is not ours to rename, so events carrying the
/// upstream sentinel are accepted as well. Rook's own TUI emits the name above.
pub const CLI_AGENT_NOTIFICATION_SENTINEL_COMPAT: &str = "rook://cli-agent";

/// Whether an OSC 777 title marks a structured CLI-agent event, from either sentinel.
pub fn is_cli_agent_notification(title: &str) -> bool {
    title == CLI_AGENT_NOTIFICATION_SENTINEL || title == CLI_AGENT_NOTIFICATION_SENTINEL_COMPAT
}

/// Schema version emitted by the current CLI-agent notification protocol.
pub const CLI_AGENT_PROTOCOL_VERSION: u32 = 1;

/// Environment variable that advertises the host's CLI-agent protocol version.
pub const ROOK_CLI_AGENT_PROTOCOL_VERSION_ENV: &str = "ROOK_CLI_AGENT_PROTOCOL_VERSION";

/// The names the published plugins read to decide whether the terminal they are running in
/// supports structured notifications. Both are exported alongside the names above: a plugin
/// that finds neither falls back to sending a plain notification instead.
pub const COMPAT_CLI_AGENT_PROTOCOL_VERSION_ENV: &str = "ROOK_CLI_AGENT_PROTOCOL_VERSION";
pub const COMPAT_CLIENT_VERSION_ENV: &str = "ROOK_CLIENT_VERSION";

/// Environment variable that identifies the hosting Rook client version.
pub const ROOK_CLIENT_VERSION_ENV: &str = "ROOK_CLIENT_VERSION";

/// Wire representation of a structured CLI-agent notification.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLIAgentNotification {
    pub v: Option<u32>,
    pub agent: Option<String>,
    pub event: String,
    pub session_id: Option<String>,
    pub cwd: Option<String>,
    pub project: Option<String>,
    pub query: Option<String>,
    pub response: Option<String>,
    pub transcript_path: Option<String>,
    pub summary: Option<String>,
    pub tool_name: Option<String>,
    pub tool_input: Option<serde_json::Value>,
    pub plugin_version: Option<String>,
    pub error_type: Option<String>,
}

impl CLIAgentNotification {
    pub fn new(agent: impl Into<String>, event: impl Into<String>) -> Self {
        Self {
            v: Some(CLI_AGENT_PROTOCOL_VERSION),
            agent: Some(agent.into()),
            event: event.into(),
            session_id: None,
            cwd: None,
            project: None,
            query: None,
            response: None,
            transcript_path: None,
            summary: None,
            tool_name: None,
            tool_input: None,
            plugin_version: None,
            error_type: None,
        }
    }
}
