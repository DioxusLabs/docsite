# Create a Project

Once you have the Dioxus CLI installed, you can use it to create your own project!

## Initializing a project

First, run the `dx new` command to create a new project.

> It clones this [template](https://github.com/DioxusLabs/dioxus-template), which is used to create dioxus apps.
>
> You can create your project from a different template by passing the `template` argument:
> ```
> dx new --template gh:dioxuslabs/dioxus-template
> ```

Next, navigate into your new project using `cd project-name`, or simply opening it in an IDE.

> Make sure the WASM target is installed before running the projects.
> You can install the WASM target for rust using rustup:
> ```
> rustup target add wasm32-unknown-unknown
> ```

Finally, serve your project with `dx serve`! The CLI will tell you the address it is serving on, along with additional
info such as code warnings.
