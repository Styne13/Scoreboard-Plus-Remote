# Scoreboard Plus Remote Control
### - a remote control for Scoreboard Plus written in rust

This remote control implements a simple webui to access the remote control features of [Scoreboard Plus](https://github.com/riscyseven/Scoreboard-Plus).

## Build (linux)
```
cargo build
```

## Build (windows)
```
sudo apt install gcc-mingw-w64
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu
```

## Build (macOS)
### Intel
```
docker run --rm \
    --volume "${PWD}":/root/src \
    --workdir /root/src \
      joseluisq/rust-linux-darwin-builder:latest \
        sh -c "cargo build --release --target x86_64-apple-darwin"
```
### MX (arm)
```
docker run --rm \
    --volume "${PWD}":/root/src \
    --workdir /root/src \
      joseluisq/rust-linux-darwin-builder:latest \
        sh -c "cargo build --release --target aarch64-apple-darwin"
```
