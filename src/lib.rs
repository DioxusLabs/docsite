use dioxus::prelude::*;

pub mod components;
pub mod icons;
pub mod sitemap;

#[derive(PartialEq, Props)]
pub struct AppProps {
    pub route: &'static str,
}

pub struct AppRoute(&'static str);

pub static App: FC<AppProps> = |(cx, props)| {
    use components::*;

    use_provide_state(cx, || AppRoute(props.route));
    let url = use_shared_state::<AppRoute>(cx)?;

    let body = match url.read().0 {
        "blog" => rsx!(cx, Blog {}),
        _ => rsx!(cx, Home {}),
    };

    cx.render(rsx! {
        link { href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css", rel: "stylesheet" }
        link { href: "https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.0.0/github-markdown-light.min.css", rel: "stylesheet" }
        style { {[include_str!("./components/prism/prism.css")]} }
        style { {[mdstyle]} }
        div {
            NavBar {}
            {body}
            Footer {}
        }
        script { {[include_str!("./components/prism/prism.js")]} }
    })
};

const mdstyle: &str = r#"
	.markdown-body {
		box-sizing: border-box;
		min-width: 200px;
		max-width: 980px;
		margin: 0 auto;
		padding: 45px;
        list_style: disc;
	}

	@media (max-width: 767px) {
		.markdown-body {
			padding: 15px;
		}
	}
"#;
