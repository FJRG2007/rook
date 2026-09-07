#!/bin/bash
# Hook script for Codex SessionStart event.
# Emits plugin version via rook://cli-agent so Rook can track the session.

set -euo pipefail

# Bump on every release; keep in sync with Rook's Codex plugin manager.
PLUGIN_VERSION="0.4.0"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/should-use-structured.sh"

if ! should_use_structured; then
    exit 0
fi

source "$SCRIPT_DIR/build-payload.sh"

INPUT=$(cat)
BODY=$(build_payload "$INPUT" "session_start" \
    --arg plugin_version "$PLUGIN_VERSION")
"$SCRIPT_DIR/rook-notify.sh" "rook://cli-agent" "$BODY"
