# Dvorak's Epic Games Launcher

**Not Completed Yet**

**NOT FOR ILLEGAL USE**

I try to make a Epic Games Launcher that has better experience

build by Rust, Leptos, Tauri, Tailwindcss

## Prerequisites

- Rust toolchain with nightly
- npm
- [Leptos](https://leptos.dev/)
- [Tauri](https://tauri.app/)

```bash
# Tauri CLI
cargo install tauri-cli

# Rust nightly (required by Leptos)
rustup toolchain install nightly --allow-downgrade

# WASM target
rustup target add wasm32-unknown-unknown

# Trunk WASM bundler
cargo install trunk

# `wasm-bindgen` for Apple M1 chips (required by Trunk)
cargo install wasm-bindgen-cli

# `esbuild` as dependency of `tauri-sys` crate (used in UI)
npm install --global --save-exact esbuild
```

## First Develop

run `npm i` at directory './src-ui',

then run `cargo tauri dev` at root directory

## Backend

The backend in directory /backend, it will simulate a Epic Server for the launcher

Backend will use sqlite as database, feel free add some data you want

Just run: `cargo r` or `docker compose up` in Docker

## Demonstration

- [Bilibili](https://www.bilibili.com/video/BV1pr421t7kA)
- [Youtube](https://youtu.be/I7lbnTBLCnU)

![Demonstration](https://raw.githubusercontent.com/dvorakchen/dvorak-epic-games-launcher/main/docs/epic-games-launcher.webp)

# License

MIT
