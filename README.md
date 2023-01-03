```
cargo run | jq -r .

cargo bench
CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --root --bench expansion
```
