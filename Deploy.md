1. make sure the version is bumped, in cargo.toml and in the readme
2. build docs locally and check
   $ cargo doc --all-features --no-deps -p bevy_spatial
3. make sure wasm works
   $ cargo run --target wasm32-unknown-unknown --example distance2d --features kdtree --release
4. publish the crate
   $ cargo publish
5. create github release
