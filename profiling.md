# How to profile

nix-shell -p linuxPackage.perf flameGraph
perf record --call-graph dwarf ./target/release/ray-trace
perf script | stackcollapse-perf.pl | rust-unmangle | flamegraph.pl > flame.svg
