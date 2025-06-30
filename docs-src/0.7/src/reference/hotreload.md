# Dioxus Hot-Reloading Reference

The Dioxus Hot-Reload is very powerful. When used properly, it is by-far the fastest tool to build apps with Rust.

Dioxus 0.5 featured a slightly limited form of hot-reloading while Dioxus 0.6 drastically improved it.

> Currently Dioxus cannot hot-reload *Rust* code, only RSX markup. Usually, modifying Rust code requires a full rebuild.

We provide this text guide as a resource for the details of hot-reloading. This guide also has an accompanying video as well:


<iframe style="width: 100%" height="500px" class="centered-overflow" src="https://www.youtube.com/embed/Q4Xzz8OJEoc" title="Dioxus 0.6" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>


## What can be hot-reloaded?

Within RSX, all elements and their properties can be hot-reloaded.

```rust
rsx! {
    div {}
}
```


## What causes a full-rebuild?


