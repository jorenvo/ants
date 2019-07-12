Todo
----
- Check all .clone() calls and derive Copy where it makes sense 

Pre-commit hook
---------------
#!/usr/bin/env bash
set -euo pipefail

cargo fmt -- --check
