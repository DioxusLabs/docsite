# Installation

## Install the stable version (recommended)

```
cargo install dioxus-cli@0.5.0-alpha.0
```

If you get an OpenSSL error on installation, ensure the dependencies listed [here](https://docs.rs/openssl/latest/openssl/#automatic) are installed.

## Install the latest development build through git

To get the latest bug fixes and features, you can install the development version from git. However, this is not fully tested. That means you're probably going to have more bugs despite having the latest bug fixes.

```
cargo install --git https://github.com/DioxusLabs/dioxus dioxus-cli
```

This will download the CLI from the master branch, and install it in Cargo's global binary directory (`~/.cargo/bin/` by default).

Run `dx --help` for a list of all the available commands. Furthermore, you can run `dx <command> --help` to get help with a specific command.
