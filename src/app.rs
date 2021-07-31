static App: FC<()> = |cx| {
    use_initialize(cx);

    let url = use_state(cx, || "");

    use components::*;
    let body = match *url {
        "community" => rsx!(in cx, Community {}),
        "tutorial" => rsx!(in cx, Tutorial {}),
        "blog" => rsx!(in cx, Blog {}),
        "docs" => rsx!(in cx, Docs {}),
        _ => rsx!(in cx, Home {}),
    };

    cx.render(rsx! {
        link { href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css", rel: "stylesheet" }
        style { {[include_str!("./components/prism/prsim.css")]} }
        div {
            NavBar {}
            {body}
            Footer {}
        }
        script { {[include_str!("./components/prism/js.js")]} }
    })
};
