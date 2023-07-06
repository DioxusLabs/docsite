In this guide, you'll learn to use Dioxus to build user interfaces that run anywhere. We will recreate the hackernews homepage in Dioxus:

```inject-dioxus
DemoFrame {
    hackernews_complete::App {}
}
```

First, lets setup our dependencies. In addition to the dependencies you added in the [getting started](../getting_started/index.md) guide for your platform, we need to set up a few more dependencies to work with the hacker news API:

```sh
cargo add chrono --features serde
cargo add futures
cargo add reqwest
cargo add serde --features derive
cargo add serde --features derive
cargo add serde_json
cargo add async_recursion
```
