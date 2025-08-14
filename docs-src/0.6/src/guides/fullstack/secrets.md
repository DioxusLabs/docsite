# Managing Secrets

In Dioxus fullstack applications, there are two types of secrets:

1. **Server-side secrets**: These are sensitive values like database credentials or API keys that should never be exposed to the client.

1. **Client-side secrets**: These are values needed by the client application at compile time, such as API keys.

## Server-Side Secrets

Server-side secrets are intended to remain confidential and are only accessible within the server environment. To manage these secrets:

1. **Create a `.env` file**: At the root of your Dioxus project, create a `.env` file to store your server-side secrets. For example:

   ```sh
   MONGODB_DB_NAME=your_database_name
   ```

1. **Load the `.env` file in your server entry point**: Use the [`dotenv`](https://docs.rs/dotenv) crate to load the environment variables. In your `main` entry point:

   ```rust
   use dotenv::dotenv;
   use std::env;

   #[cfg(not(feature = "web"))]
   #[tokio::main]
   async fn main() {
       dotenv().ok();

       // Access your environment variables
       let db_name = env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set.");
       // Initialize your server with the db_name or other configurations
   }
   ```

This approach ensures that sensitive information is not hardcoded and can be managed securely outside of your source code.

## Client-Side Secrets

Client-side secrets are values that need to be embedded into the client application at compile time. Since browsers do not have access to environment variables at runtime, these values must be provided during the build process.

1. **Set environment variables during the build**: When building your client application, pass the necessary environment variables. For example:

   ```bash
   STRIPE_PRICE_ID=price_12345 dx build --release
   ```

1. **Access the variables in your client code**: Use the [`env!`](https://doc.rust-lang.org/std/macro.env.html) macro to embed the environment variable values into your code at compile time:

   ```rust
   #[component]
    fn App() -> Element {
        let price_id = env!("STRIPE_PRICE_ID");

        rsx!{
        }
    }
   ```

This method ensures that the required values are available in the client application without exposing sensitive information at runtime.

## Important Considerations

- **Security**: Never expose sensitive server-side secrets to the client. Only include non-sensitive information in client-side environment variables.

- **Compile-Time Availability**: Ensure that all necessary environment variables are set during the build process to avoid compilation errors.

- **Consistency**: Maintain a clear distinction between server-side and client-side secrets to prevent accidental exposure of sensitive information.

By following these practices, you can manage secrets effectively in your Dioxus fullstack application, maintaining both security and functionality.
