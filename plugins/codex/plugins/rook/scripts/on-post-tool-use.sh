#!/bin/bash
# Hook script for Codex PostToolUse event.
# Sends a structured Rook notification after a tool call completes.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/should-use-structured.sh"

if ! should_use_structured; then
    exit 0
fi

source "$SCRIPT_DIR/build-payload.sh"

INPUT=$(cat)
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name // empty' 2>/dev/null)

BODY=$(build_payload "$INPUT" "tool_complete" \
    --arg tool_name "$TOOL_NAME")

"$SCRIPT_DIR/rook-notify.sh" "rook://cli-agent" "$BODY"
