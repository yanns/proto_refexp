```
# run once
cargo run | jq -r .

# run the tests
cargo test
cargo watch -x test

# run the benchmark
cargo bench

# open the HTLM report
open target/criterion/report/index.html

# run the benchmark and get a flamegraph ('--root' for mac)
CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --root --bench expansion

# open the flamegraph
open flamegraph.svg
```
