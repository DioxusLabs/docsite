# Setting up Tooling

Before we get started, make sure you've followed the [Getting Started](../getting_started/index.md) page on installing the required dependencies.

We will be primarily developing *HotDog* as web application, but we still recommend setting up the relevant tooling for desktop and mobile development as well.

## Checklist

We covered the setup instructions in [Getting Started](../getting_started/index.md), but first you should verify everything is set up properly:

- Rust is installed
- You have a code editor installed
- The wasm32-unknown-unknown Rust toolchain is installed
- The `dioxus-cli` is installed and up-to-date
- System-specific dependencies are installed

## Verify your setup

Before proceeding, make sure you have the `dioxus-cli` installed and up-to-date.

Verify the returned version matches this guide (eg 0.6) by running:

```sh
dx --version
```

## All the Commands

You can also run `dx help` which will give you a list of useful commands and some information on how to use `dx`.

```sh
Build, Bundle & Ship Dioxus Apps

Usage: dx [OPTIONS] <COMMAND>

Commands:
  build      Build the Dioxus project and all of its assets
  translate  Translate a source file into Dioxus code
  serve      Build, watch & serve the Dioxus project and all of its assets
  new        Create a new project for Dioxus
  init       Init a new project for Dioxus in the current directory (by default). Will attempt to keep your project in a good state
  clean      Clean output artifacts
  bundle     Bundle the Dioxus app into a shippable object
  fmt        Automatically format RSX
  check      Check the project for any issues
  run        Run the project without any hotreloading
  doctor     Ensure all the tooling is installed and configured correctly
  config     Dioxus config file controls
  help       Print this message or the help of the given subcommand(s)

Options:
      --verbose      Use verbose output [default: false]
      --trace        Use trace output [default: false]
      --json-output  Output logs in JSON format
  -h, --help         Print help
  -V, --version      Print version
```

If `dx` is installed properly, then you're ready to proceed!
