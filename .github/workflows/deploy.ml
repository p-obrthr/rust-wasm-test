name: Build and Deploy

on:
  push:
    branches:
      - main

permissions:
  contents: write

jobs:
  deploy:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-emscripten

      - name: Install Emscripten
        uses: mymindstorm/setup-emsdk@v14

      - name: Build
        run: |
          cargo build --release --target wasm32-unknown-emscripten

      - name: Prepare Pages
        run: |
          mkdir public
          cp target/wasm32-unknown-emscripten/release/index.html public/
          cp target/wasm32-unknown-emscripten/release/*.js public/
          cp target/wasm32-unknown-emscripten/release/*.wasm public/

      - name: Deploy
        uses: peaceiris/actions-gh-pages@v4
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./public
