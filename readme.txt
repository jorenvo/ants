Todo
----
- Check all .clone() calls and derive Copy where it makes sense, also get rid of &
- Make calc_random_direction faster when ant is in corner

Pre-commit hook
---------------
#!/usr/bin/env bash
set -euo pipefail

cargo fmt -- --check
