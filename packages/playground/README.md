## Dioxus Playground
Dioxus Playground is a code playground similar to the Rust playground except it is tailored specifically for Dioxus and it's features.

### Design
The playground is in a Cargo workspace with:
- `playground`: the playground component library.
- `server`: an axum server that runs the build logic.
- `model`: shared structures for communicating between web and server.
- `runner`: currently wraps around the playground component to run it.

**Build Flow:**
1. User submits code
2. The code is added to a compilation queue
3. The server compiles the code one at a time with the same cached dependencies. (dioxus)
4. The server stores the compiled wasm and provides a temporary url to the page.
5. The server deletes the wasm after a duration from being built.


### Component Usage
Simply include and use the `Playground {}` component wherever you want. 
If you want share functionality, you will need to wire up Dioxus router for `playgroundUrlPrefix/:share_code`.

### Share Functionality
The share functionality requires no server. It simply takes the code, compresses it with `miniz_oxide`, and Base64 encodes it.

### Server Environment Variables
Most of these are already set in the `Dockerfile` and shouldn't need modified.
```
# The port the server should listen to.
PORT = 3000

# Whether the server should run in produce mode. This informs the server to use forwarded ip addresses.
PRODUCTION = false

# The build template that should be used.
BUILD_TEMPLATE_PATH = "/usr/local/bin/template"

# If specified, shuts down the server after X ms since the last http request.
SHUTDOWN_DELAY = null

# The authentication token for saving code to GitHub gist.
GIST_AUTH_TOKEN = null
```

### Licenses
All `dioxus-playground` code is licensed under [MIT](./LICENSE-MIT) or [Apache 2.0](./LICENSE-APACHE). 

This project includes 3rd party assets which may have their own licensing requirements. For more information, see the [licenses readme](./LICENSES/README.md).