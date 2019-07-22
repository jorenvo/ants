Todo
----
- Check all .clone() calls and derive Copy where it makes sense, also get rid of &
- Remove pheromone_generation

Pre-commit hook
---------------
#!/usr/bin/env bash
set -euo pipefail

cargo fmt -- --check

Sources
-------
- Ant Colony Optimization: Artificial Ants as a Computational Intelligence Technique
  https://courses.cs.ut.ee/all/MTAT.03.238/2011K/uploads/Main/04129846.pdf
