Pre-commit hook
---------------
#!/usr/bin/env bash
set -euo pipefail

cargo fmt -- --check
