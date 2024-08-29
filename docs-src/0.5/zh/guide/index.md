# Dioxus Guide

## Introduction

In this guide, you'll learn to use Dioxus to build user interfaces that run anywhere. We will recreate the hackernews homepage in Dioxus:

\```inject-dioxus
DemoFrame {
    hackernews_complete::App {}
}
\```

This guide serves a very brief overview of Dioxus. Throughout the guide, there will be links to the [reference](../reference/index.md) with more details about specific concepts.

## Setup
Before you start the guide, make sure you have the dioxus CLI and a project set up for your platform as described in the [getting started](../getting_started/index.md) guide.

First, lets setup our dependencies. In addition to the dependencies you added in the [getting started](../getting_started/index.md) guide for your platform, we need to set up a few more dependencies to work with the hacker news API:

\```sh
cargo add chrono --features serde
cargo add futures
cargo add reqwest --features json
cargo add serde --features derive
cargo add serde_json
cargo add async_recursion
\```
