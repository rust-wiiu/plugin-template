# Homebrew Plugins

This template helps you get started with writing plugins for the Aroma Homebrew environment.

For more information take a look at [Book for U](https://rust-wiiu.github.io/book-for-u/).

## How to use

### [cargo-generate](https://github.com/cargo-generate/cargo-generate)

```
cargo generate rust-wiiu/plugin-template
```

### git clone

```
git clone https://github.com/rust-wiiu/plugin-template.git
```

Modify `author` and `authors` in `Cargo.toml`

## How to run


Build the binary (.wps) with
```
cargo make build
```

Build and upload the binray via wiiload with
```
cargo make run
```
