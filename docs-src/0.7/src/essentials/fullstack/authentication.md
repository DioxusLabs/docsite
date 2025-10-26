# Authentication

Most production-ready apps have some sort of authentication and authorization to restrict data and resources from being publicly accessible to all users.

Dioxus does *not* provide a built-in way of managing authentication. However, since auth is such a critical component of building fullstack apps, this chapter will help guide you through the process of implementing it for your app.

> In the future, we'd like to integrate sessions and auth directly into Dioxus fullstack, but for now, you'll need to rely on 3rd-party libraries.

## What is Authentication?

Authentication is the process of verifying a user's identity, usually through the use of cookies or access tokens.

Your app should not blindly accept actions from untrusted users, so it's important to verify that a user who claims to be "Bob" *actually is* "Bob".

## What is Authorization?

Authorization is the *next step* after authentication. Once a user's identity is verified, you must then verify that they can actually take a given action.

You don't want one user to change another user's login credentials, so any action related to user data must be *authorized* first.

## Auth is Built on Sessions

Fundamentally, auth works by associating a users *connection* with some *session* in a database. As a user traverses our page and makes actions, we need some way of identifying their session on the server.

Usually, this is done via a middleware that automatically registers every connection with a row in your database. If the user is *unauthenticated*, you can either store their session as an anonymous user, or a ignore it competely.

A session is usually inserted with an axum-level middleware:

```rust
dioxus::server::router(app)
    .layer(axum::middleware::from_fn(
        |request: Request, next: Next| async move {
            // Get the auth token and then insert the session into the request's extensions
            //
            // We need to look up the auth token in our database to retrieve the session
            if let Some(token) = request.headers().get("Authorization") {
                if let Some(session) = DATABASE.get_session(token) {
                    request.extensions_mut().insert(session);
                }
            }

            // And then run the request
            next.run(request).await
        },
    ))
```

## Libraries for Authentication

You can implement session lookup and session creation system manually, or use a 3rd-party library like [`axum_session_auth`](https://crates.io/crates/axum_session_auth) to handle most of the complexity for you.

Generally, you'll add a 3rd-party library as a `.layer()` on your router, and it will add the `Session` extension to your requests automatically:

```rust
dioxus::serve(|| async move {
    // Create an axum router that dioxus will attach the app to
    Ok(dioxus::server::router(app)
        // Add the `AuthLayer`
        .layer(
            AuthLayer::new(Some(db.clone()))
                .with_config(AuthConfig::<i64>::default().with_anonymous_user_id(Some(1))),
        )
        // And add the `SessionLayer`
        .layer(SessionLayer::new(
            SessionStore::<SessionSqlitePool>::new(
                Some(db.into()),
                SessionConfig::default().with_table_name("test_table"),
            )
            .await?,
        )))
})
```

The `axum_session_auth` crate integrates with `Sqlx` to automatically manage sessions using a row in your database.


## Auth as an Extractor

You can use extractors to integrate auth with your Fullstack application.

You can create a custom extractor to extract the auth session from the request. From that auth session, you can check if the user has the required privileges before returning the private data.

Because the sessions are inserted into every request, you can extract them with server-only extractors:

```rust
/// We use the `auth::Session` extractor to get access to the current user session.
/// This lets us modify the user session, log in/out, and access the current user.
#[post("/api/user/login", auth: auth::Session)]
pub async fn login() -> Result<()> {
    auth.login_user(2);
    Ok(())
}

/// Just like `login`, but this time we log out the user.
#[post("/api/user/logout", auth: auth::Session)]
pub async fn logout() -> Result<()> {
    auth.logout_user();
    Ok(())
}
```

A [full auth example](https://github.com/DioxusLabs/dioxus/blob/7102bc3b6a0ddea3a9e71423fc6d667df8d956f3/examples/07-fullstack/auth/src/main.rs) with the complete implementation is available in the fullstack examples.

## Auth as a 3rd-party service

Implementing auth can be somewhat tedious, so there are a number of 3rd party services that simplify the implementation for you. With these services, the session and auth data tends to not live in *your database*, but rather theirs.

We don't recommend any particular 3rd party service. However, there are some solutions like Supabase and Firebase where sessions are managed in your database, but with prebuilt infrastructure for you to integrate with.
