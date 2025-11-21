1. make sure the version is bumped, in cargo.toml and in the readme
2. build docs locally and check
   $ cargo doc --all-features --no-deps -p bevy_spatial
3. make sure wasm works
   $ bevy run --example distance2d --release --no-default-features --features kdtree web --open
4. publish the crate
   $ cargo publish
5. create github release
