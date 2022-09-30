# everywhere

Make your rust code run everywhere with one script.

```sh
./build.sh
```

Builds:

- NodeJS `out/everywhere.node`, `out/everywhere.d.ts` as seen in `package.json`
- Windows `out/everywhere.exe`
- Linux `out/everywhere`
- Rust `/` as seen in `Cargo.toml`
- Web `out/everywhere.wasm`, `out/everywhere.js`

The lib code resides in `src/index.rs`, the bin in `src/main.rs`.
