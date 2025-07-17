#![allow(unused)]

// Include any examples we compile into the docsite
#[cfg(not(feature = "doc_test"))]
pub mod boolean_attribute;
#[cfg(not(feature = "doc_test"))]
pub mod component_borrowed_props;
#[cfg(not(feature = "doc_test"))]
pub mod component_children;
#[cfg(not(feature = "doc_test"))]
pub mod component_owned_props;
#[cfg(not(feature = "doc_test"))]
pub mod components;
#[cfg(not(feature = "doc_test"))]
pub mod conditional_rendering;
#[cfg(not(feature = "doc_test"))]
pub mod dangerous_inner_html;
#[cfg(not(feature = "doc_test"))]
pub mod event_click;
#[cfg(not(feature = "doc_test"))]
pub mod event_prevent_default;
#[cfg(not(feature = "doc_test"))]
pub mod full_example;
#[cfg(not(feature = "doc_test"))]
pub mod hackernews_async;
#[cfg(not(feature = "doc_test"))]
pub mod hackernews_complete;
#[cfg(not(feature = "doc_test"))]
pub mod hackernews_post;
#[cfg(not(feature = "doc_test"))]
pub mod hackernews_state;
#[cfg(not(feature = "doc_test"))]
pub mod hello_world;
#[cfg(not(feature = "doc_test"))]
pub mod hooks_counter;
#[cfg(not(feature = "doc_test"))]
pub mod hooks_counter_two_state;
#[cfg(not(feature = "doc_test"))]
pub mod hooks_out_of_date;
#[cfg(not(feature = "doc_test"))]
pub mod hooks_use_ref;
#[cfg(not(feature = "doc_test"))]
pub mod input_controlled;
#[cfg(not(feature = "doc_test"))]
pub mod input_uncontrolled;
#[cfg(not(feature = "doc_test"))]
pub mod readme;
#[cfg(not(feature = "doc_test"))]
pub mod rendering_lists;
#[cfg(not(feature = "doc_test"))]
pub mod rsx_overview;
#[cfg(not(feature = "doc_test"))]
pub mod spawn;
#[cfg(not(feature = "doc_test"))]
pub mod use_future;

// Check any examples we don't compile into the docs
#[cfg(feature = "doc_test")]
automod::dir!(pub "src/doc_examples");
