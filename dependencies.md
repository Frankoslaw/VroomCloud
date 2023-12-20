NVM
```sh
wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
nvm install node
nvm use node
```

PNPM
```sh
wget -qO- https://get.pnpm.io/install.sh | sh -
```

RUST
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

AYA-RS
```sh
sudo dnf install bpftool netcat git wget
rustup install stable
rustup toolchain install nightly --component rust-src
cargo install bpf-linker
cargo install cargo-generate
cargo install bindgen-cli
```
