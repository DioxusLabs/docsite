# Authentication

You can use [extractors](./extractors) to integrate auth with your Fullstack application.

You can create a custom extractors that extracts the auth session from the request. From that auth session, you can check if the user has the required privileges before returning the private data.

A [full auth example](https://github.com/DioxusLabs/dioxus/blob/v0.5/packages/fullstack/examples/axum-auth/src/main.rs) with the complete implementation is available in the fullstack examples.
