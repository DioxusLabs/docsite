use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    Page,
    Logs,
}

#[component]
pub fn RightPane(
    current_tab: Signal<Tab>,
    compiler_messages: Signal<Vec<String>>,
    built_page_url: Option<String>,
) -> Element {
    let mut pane_is_mounted = use_signal(|| false);

    use_memo(move || {
        _ = compiler_messages();
        if !pane_is_mounted() {
            return;
        }

        // Scrolls to the bottom of the log pane
        eval(
            r#"
            let pane = document.getElementById("dxp-right-pane");
            pane.scrollTop = pane.scrollHeight;
            "#,
        );
    });

    let logs = use_memo(move || {
        let mut logs = Vec::new();
        let mut error_span = false;

        for log in compiler_messages.iter() {
            let is_success = log.to_lowercase().contains("build done");
            let is_err = log.to_lowercase().contains("error");
            let is_dep = log.contains('⚙');
            if !error_span && !is_dep {
                error_span = is_err;
            }

            if log.is_empty() {
                error_span = false;
            }

            let log_rsx = rsx! {
                p {
                    class: "dxp-log-message",
                    class: if is_success { "dxp-log-success" },
                    class: if error_span { "dxp-log-error" },

                    "{log}"
                }
            };

            logs.push(log_rsx);
        }

        logs
    });

    rsx! {
        div {
            id: "dxp-right-pane",

            onmounted: move |_| pane_is_mounted.set(true),

            match current_tab() {
                Tab::Page => rsx! {
                    iframe {
                        src: built_page_url,
                    }
                },
                Tab::Logs => rsx! { {logs().iter()} },
            }

        }
    }
}
