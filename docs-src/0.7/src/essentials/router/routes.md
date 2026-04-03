# Defining Routes

When creating a `Routable` enum, we can define routes for our application using the `route("path")` attribute.

## Route Segments

Each route is made up of segments. Most segments are separated by `/` characters in the path.

There are five fundamental types of segments:

1. [Static segments](#static-segments) are fixed strings that must be present in the path.
2. [Dynamic segments](#dynamic-segments) are types that can be parsed from a segment.
3. [Catch-all segments](#catch-all-segments) are types that can be parsed from multiple segments.
4. [Query segments](#query-segments) are types that can be parsed from the query string.
4. [Hash fragments](#hash-segments) are types that can be parsed from the hash fragment.

Routes are matched:

- First, from most specific to least specific (Static then Dynamic then Catch All) (Query and hash are always matched)
- Then, if multiple routes match the same path, the order in which they are defined in the enum is followed.

## Static segments

Fixed routes match a specific path. For example, the route `#[route("/about")]` will match the path `/about`.

```rust
{{#include ../docs-router/src/doc_examples/static_segments.rs:route}}
```

## Dynamic Segments

Dynamic segments are in the form of `:name` where `name` is
the name of the field in the route variant. If the segment is parsed
successfully then the route matches, otherwise the matching continues.

The segment can be of any type that implements `FromStr`.

```rust
{{#include ../docs-router/src/doc_examples/dynamic_segments.rs:route}}
```

<details>
<summary>Custom parsing with your own types</summary>

Any type that implements `FromStr` + `Display` can be used as a dynamic segment. If parsing fails, the route won't match and the router moves on to the next candidate. This lets you restrict which URLs match a route — for example, only accepting known locales:

```rust
{{#include ../docs-router/src/doc_examples/route_customization.rs:custom_dynamic_segment}}
```

With this route, `/en/about` and `/fr/about` will match, but `/xyz/about` will not.

See [`FromRouteSegment`](https://docs.rs/dioxus-router/latest/dioxus_router/routable/trait.FromRouteSegment.html) on docs.rs for the full trait definition.

</details>

## Catch All Segments

Catch All segments are in the form of `:..name` where `name` is the name of the field in the route variant. If the segments are parsed successfully then the route matches, otherwise the matching continues.

The segment can be of any type that implements `FromSegments`. (`Vec<String>` implements this by default)

Catch All segments must be the _last route segment_ in the path (query segments are not counted) and cannot be included in nests.

```rust
{{#include ../docs-router/src/doc_examples/catch_all_segments.rs:route}}
```

<details>
<summary>Custom parsing for catch-all segments</summary>

By default, `Vec<String>` collects catch-all segments. You can implement [`FromRouteSegments`](https://docs.rs/dioxus-router/latest/dioxus_router/routable/trait.FromRouteSegments.html) directly to parse the segments into a structured type:

```rust
{{#include ../docs-router/src/doc_examples/route_customization.rs:custom_catch_all}}
```

</details>

## Query Segments

Query segments are in the form of `?:name&:othername` where `name` and `othername` are the names of fields in the route variant.

Unlike [Dynamic Segments](#dynamic-segments) and [Catch All Segments](#catch-all-segments), parsing a Query segment must not fail.

The segment can be of any type that implements `FromQueryArgument`.

Query segments must be the _after all route segments_ and cannot be included in nests.

```rust
{{#include ../docs-router/src/doc_examples/query_segments.rs:route}}
```

<details>
<summary>Custom query parameter parsing</summary>

Individual query parameters use the [`FromQueryArgument`](https://docs.rs/dioxus-router/latest/dioxus_router/routable/trait.FromQueryArgument.html) trait, which is auto-implemented for any `FromStr + Default` type. If the parameter is missing or fails to parse, `Default::default()` is used instead of failing the route.

You can use your own types as query parameters by implementing `FromStr`, `Default`, and `Display`:

```rust
{{#include ../docs-router/src/doc_examples/route_customization.rs:custom_single_query}}
```

If you need full control over the entire query string — for example, to handle dynamic keys or custom serialization — you can capture it into one type using the spread syntax `?:..field`. The type must implement [`From<&str>`](https://docs.rs/dioxus-router/latest/dioxus_router/routable/trait.FromQuery.html) and `Display`:

```rust
{{#include ../docs-router/src/doc_examples/route_customization.rs:custom_spread_query}}
```

</details>

## Hash Segments

Hash segments are in the form of `#:field` where `field` is a field in the route variant.

Just like [Query Segments](#query-segments), parsing a Hash segment must not fail.

The segment can be of any type that implements `FromHashFragment`.

Hash fragments must be the _after all route segments and any query segments_ and cannot be included in nests.

```rust
{{#include ../docs-router/src/doc_examples/hash_fragments.rs:route}}
```

<details>
<summary>Custom parsing for hash fragments</summary>

The [`FromHashFragment`](https://docs.rs/dioxus-router/latest/dioxus_router/routable/trait.FromHashFragment.html) trait is auto-implemented for any `FromStr + Default` type. Parsing failures return `Default::default()` instead of causing the route to fail.

You can use a custom type to parse structured data from the hash fragment:

```rust
{{#include ../docs-router/src/doc_examples/route_customization.rs:custom_hash}}
```

</details>

## Nested Routes

When developing bigger applications we often want to nest routes within each
other. As an example, we might want to organize a settings menu using this
pattern:

```plain
└ Settings
  ├ General Settings (displayed when opening the settings)
  ├ Change Password
  └ Privacy Settings
```

We might want to map this structure to these paths and components:

```plain
/settings		  -> Settings { GeneralSettings }
/settings/password -> Settings { PWSettings }
/settings/privacy  -> Settings { PrivacySettings }
```

Nested routes allow us to do this without repeating /settings in every route.

### Nesting

To nest routes, we use the `#[nest("path")]` and `#[end_nest]` attributes.

The path in nest must not:

1. Contain a [Catch All Segment](routes.md#catch-all-segments)
2. Contain a [Query Segment](routes.md#query-segments)

If you define a dynamic segment in a nest, it will be available to all child routes and layouts.

To finish a nest, we use the `#[end_nest]` attribute or the end of the enum.

```rust
{{#include ../docs-router/src/doc_examples/nest.rs:route}}
```
