#!/usr/bin/env bash
set -euo pipefail

# stdio MCP サーバーを起動するヘルパ。
# 使用例:
#   ./examples/start-stdio.sh /ABS/PATH/TO/your/gowin/project

if [[ $# -lt 1 ]]; then
	echo "Usage: $0 <project_root>" >&2
	exit 2
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd -P)"
PROJECT_ROOT="$1"

export GOWIN_MCP_PROJECT_ROOT="$PROJECT_ROOT"
cd "$REPO_ROOT"
exec "$REPO_ROOT/target/release/gw-synth-flash-mcp"
