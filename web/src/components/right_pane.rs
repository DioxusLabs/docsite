use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    Page,
    Logs,
}

#[component]
pub fn RightPane(
    current_tab: Tab,
    built_page_uri: Option<String>,
    compiler_messages: Vec<String>,
) -> Element {
    let mut pane_is_mounted = use_signal(|| false);

    use_memo(use_reactive((&compiler_messages,), move |(_messages,)| {
        if !pane_is_mounted() {
            return;
        }

        eval(
            r#"
            let pane = document.getElementById("right-pane");
            pane.scrollTo({ top: pane.scrollHeight, behavior: 'smooth' });
            "#
        );
    }));

    let mut logs = Vec::new();
    let mut error_span = false;

    for log in compiler_messages.iter() {
        let is_success = log.to_lowercase().contains("build done");
        let is_err = log.to_lowercase().contains("error");
        let is_dep = log.contains("⚙");
        if !error_span && !is_dep {
            error_span = is_err;
        }

        if log.is_empty() {
            error_span = false;
        }

        let log_rsx = rsx! {
            p {
                class: "log-message",
                class: if is_success { "log-success" },
                class: if error_span { "log-error" },
                "{log}"
            }
        };

        logs.push(log_rsx);
    }

    rsx! {
        div {
            id: "right-pane",
            onmounted: move |_| pane_is_mounted.set(true),

            match current_tab {
                Tab::Page => rsx! {
                    iframe {
                        src: built_page_uri,
                    }
                },
                Tab::Logs => rsx! { {logs.iter()} },
            }

        }
    }
}
