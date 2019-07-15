Todo
----
- Check all .clone() calls and derive Copy where it makes sense, also get rid of &
- Add a "going home" pheromone when carrying food, should be preferred over home base pheromone

Pre-commit hook
---------------
#!/usr/bin/env bash
set -euo pipefail

cargo fmt -- --check
