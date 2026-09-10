# Setting up Tooling

Before we get started, make sure you've followed the [Getting Started](../getting_started/index.md) page on installing the required dependencies. We will be primarily developing *HotDog* as web application, but we still recommend setting up the relevant tooling for desktop and mobile development as well.

## Checklist

We covered the setup instructions in [Getting Started](../getting_started/index.md), but first verify your setup:

- Rust and Cargo are installed
- The wasm32-unknown-unknown Rust target is installed
- The `dioxus-cli` is installed and up-to-date
- System-specific dependencies are installed

Refer to the `dx doctor` command to see what `dx` uses to build your app.

## All the Commands

Before proceeding, make sure you have the `dioxus-cli` installed and up-to-date. Verify the returned version matches this guide by running:

```sh
dx --version
```

You can also run `dx help` which will give you a list of useful commands and some information on how to use `dx`.

```sh
Dioxus: build web, desktop, and mobile apps with a single codebase

Usage: dx [OPTIONS] <COMMAND>

Commands:
  new          Create a new Dioxus project
  serve        Build, watch, and serve the project
  bundle       Bundle the Dioxus app into a shippable object
  build        Build the Dioxus project and all of its assets
  run          Run the project without any hotreloading
  init         Init a new project for Dioxus in the current directory (by default). Will attempt to keep your project in a good state
  doctor       Diagnose installed tools and system configuration
  print        Print project information in a structured format, like cargo args, linker args, and other flags DX sets that might be useful in third-party tools
  translate    Translate a source file into Dioxus code
  fmt          Automatically format RSX
  check        Check the project for any issues
  config       Dioxus config file controls
  self-update  Update the Dioxus CLI to the latest version
  tools        Run a dioxus build tool. IE `build-assets`, `hotpatch`, etc
  components   Manage components from the `dioxus-component` registry
  help         Print this message or the help of the given subcommand(s)

Options:
      --verbose      Use verbose output [default: false]
      --trace        Use trace output [default: false]
      --json-output  Output logs in JSON format
  -h, --help         Print help
  -V, --version      Print version

Logging Options:
      --log-to-file <LOG_TO_FILE>  Write *all* logs to a file

Manifest Options:
      --locked   Assert that `Cargo.lock` will remain unchanged
      --offline  Run without accessing the network
      --frozen   Equivalent to specifying both --locked and --offline
```

If `dx` is installed properly, then you're ready to proceed!
