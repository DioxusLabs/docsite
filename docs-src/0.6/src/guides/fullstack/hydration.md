# Hydration

Dioxus fullstack renders the initial HTML on the server and then continues updating that HTML on the client. Hydration lets the client pick up where the server left off. Most of the time, you shouldn't need to think about hydration, but there are a few cases where you might run into hydration issues. To better understand hydration, we'll walk through a simple example:

```rust
{{#include src/doc_examples/hydration.rs:hydration_intro}}
```

## Rendering the initial HTML

When the server receives a request to render the `Weather` component, it will:

1. Run the component and wait until all server futures are resolved
2. Serializes any non-deterministic data (like the `weather` future) to hydrate on the client
3. Renders the HTML

[![](https://mermaid.ink/img/pako:eNpdkDFTwzAMhf-KT3M70HbKwELhGMqSdAIziFhNfI2lnGzDQa__HZfk4Iq1-D1_ejr5BK04ggoOg3y0PWoy-61lE_Nbpzj2Jt68WGhI30lN4x2ZmtiReu4svBZwPs4rtckLm13958ZVaa4zmzsJozBxumqK6ynb4-C_yMxTHnLKSvGa3FyCfiabx_3Tbn4sxsTElVkub0vgLNeT3FieChYQSAN6V1Y9XSALqadAFqpydahHC5bPhcOcpPnkFqqkmRagkrseqgMOsag8Oky09Vh-J_y6I_KzSPhH3TufRGfz_A3Ce3PT?type=png)](https://mermaid-js.github.io/mermaid-live-editor/edit#pako:eNpdkDFTwzAMhf-KT3M70HbKwELhGMqSdAIziFhNfI2lnGzDQa__HZfk4Iq1-D1_ejr5BK04ggoOg3y0PWoy-61lE_Nbpzj2Jt68WGhI30lN4x2ZmtiReu4svBZwPs4rtckLm13958ZVaa4zmzsJozBxumqK6ynb4-C_yMxTHnLKSvGa3FyCfiabx_3Tbn4sxsTElVkub0vgLNeT3FieChYQSAN6V1Y9XSALqadAFqpydahHC5bPhcOcpPnkFqqkmRagkrseqgMOsag8Oky09Vh-J_y6I_KzSPhH3TufRGfz_A3Ce3PT)

Once the server finishes rendering, it will send this structure to the client serialized into the HTML:

[![](https://mermaid.ink/img/pako:eNqFUcFKAzEQ_ZUwh57agy22sAUFqaCgF1sQNCLTZLYbupss2VmLlv67s92luoo4uSRv3nvzhuzBBEuQQJqHnckwslottFdVvd5ELDNVjZ81LCk6zN0HWbVARg0vQpGyLpJhF7xaXbVIU_5MJCmxyV53hJxR7ATkbc96Irxb71i81c3q_u4_3ybKIOe5dW-DDc9P9GPzXJr7bl5yeeg3p51yXTOLa_Amd2b722QmvAdKI1XZH5mb3R57W7VVjb_dJ1_KY241Gl1IQjWQJB02bbGZ9u2BIRQUC3RWPmPfkDTIkII0JHK1GLcatD8ID2sOy3dvIOFY0xBiqDcZJCnmlbzq0iLTwqEELk5oif4phOIH69o6DrEDD5_uGqQ1?type=png)](https://mermaid-js.github.io/mermaid-live-editor/edit#pako:eNqFUcFKAzEQ_ZUwh57agy22sAUFqaCgF1sQNCLTZLYbupss2VmLlv67s92luoo4uSRv3nvzhuzBBEuQQJqHnckwslottFdVvd5ELDNVjZ81LCk6zN0HWbVARg0vQpGyLpJhF7xaXbVIU_5MJCmxyV53hJxR7ATkbc96Irxb71i81c3q_u4_3ybKIOe5dW-DDc9P9GPzXJr7bl5yeeg3p51yXTOLa_Amd2b722QmvAdKI1XZH5mb3R57W7VVjb_dJ1_KY241Gl1IQjWQJB02bbGZ9u2BIRQUC3RWPmPfkDTIkII0JHK1GLcatD8ID2sOy3dvIOFY0xBiqDcZJCnmlbzq0iLTwqEELk5oif4phOIH69o6DrEDD5_uGqQ1)

## Hydrating on the client

Once the client receives the initial HTML, it hydrates the HTML by rerunning each component and linking each element the server rendered to the corresponding element on the client. Rerunning each component lets the client re-construct some non-serializable state like event handlers and kick off any client side logic like `use_effect` and `use_future`.

It will follow these steps:

1. Deserialize the serer future data from the server
2. Run the component with the deserialized data. All server futures are immediately resolved with the deserialized data from the server.
3. Hydrate the HTML sent from the server. This adds all event handlers and links the html nodes to the component so they can be modified later

[![](https://mermaid.ink/img/pako:eNpdkLFuAjEMhl_F8gxDgemGLlyrDnThmNp0SC-Gi7g4JydpRRHvXsOdkFpnif__s534jG10hBXu-_jddlYy7GrDkMrnQezQQXp4N7juPXGGxjuCl5MT63Nkgx8Kajgv1GYfGTbbUblGWmphTYnE297_EDQkXyTwXHIRSvfqG7tQdlsY1jEMkXXWX3ul9m1u1vm7183kErsRSkuYzx-1zZQuxnRleDw4w0ASrHf60_MVMpg7CmSw0quzcjRo-KKcLTk2J26xylJohhLLocNqb_ukWRmcvqH2VpcT7upg-S3G8I96crolmcTLL4RBdIg?type=png)](https://mermaid-js.github.io/mermaid-live-editor/edit#pako:eNpdkLFuAjEMhl_F8gxDgemGLlyrDnThmNp0SC-Gi7g4JydpRRHvXsOdkFpnif__s534jG10hBXu-_jddlYy7GrDkMrnQezQQXp4N7juPXGGxjuCl5MT63Nkgx8Kajgv1GYfGTbbUblGWmphTYnE297_EDQkXyTwXHIRSvfqG7tQdlsY1jEMkXXWX3ul9m1u1vm7183kErsRSkuYzx-1zZQuxnRleDw4w0ASrHf60_MVMpg7CmSw0quzcjRo-KKcLTk2J26xylJohhLLocNqb_ukWRmcvqH2VpcT7upg-S3G8I96crolmcTLL4RBdIg)

## Hydration errors

For hydration to work, **the component must render exactly the same thing on the client and the server**. If it doesn't, you might see an error like this:

```
Uncaught TypeError: Cannot set properties of undefined (setting 'textContent')
at RawInterpreter.run (yourwasm-hash.js:1:12246)
```

Or this:

```
Error deserializing data:
Semantic(None, "invalid type: floating point `1.2`, expected integer")
This type was serialized on the server at src/main.rs:11:5 with the type name f64. The client failed to deserialize the type i32 at /path/to/server_future.rs
```

### Non-deterministic data with server cached

To avoid hydration errors, make sure you put any non-deterministic data in a `use_server_future`, `use_server_cached` or effect. For example, if you need
to render a random number on your page, you can use `use_server_cached` to cache the random number on the server and then use it on the client:

```rust
{{#include src/doc_examples/hydration.rs:server_cached}}
```

### Async loading with server futures

If you need render some data from a server future, you need to use `use_server_future` to serialize the data instead of waiting for the (non-deterministic) amount of time `use_resource(...).suspend()?` takes:

```rust
{{#include src/doc_examples/hydration.rs:server_future}}
```

### Client only data with effects

If you need to grab some data that is only available on the client, make sure you get it inside of a
`use_effect` hook which runs after the component has been hydrated:

```rust
{{#include src/doc_examples/hydration.rs:effects}}
```

### Avoid side effects in server cached hooks

The dioxus fullstack specific hooks `use_server_cached` and `use_server_future` don't run the same on the server and the client. The code you run inside these hooks cannot have side effects because those side effects cannot be serialized:

```rust
{{#include src/doc_examples/hydration.rs:server_hook_side_effects}}
```
