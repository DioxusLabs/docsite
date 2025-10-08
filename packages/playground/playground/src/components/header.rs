use crate::build::{BuildStage, BuildState};
use crate::components::icons::LoadingSpinner;
use crate::dx_components::button::*;
use crate::dx_components::select::*;
use crate::share_code::copy_share_link;
use crate::HotReload;
use crate::{Errors, PlaygroundUrls};
use dioxus::prelude::*;
use model::api::ApiClient;
use model::Project;

#[component]
pub fn Header(
    urls: PlaygroundUrls,
    on_rebuild: EventHandler,
    pane_left_width: Signal<Option<i32>>,
    pane_right_width: Signal<Option<i32>>,
    mut show_examples: Signal<bool>,
    file_name: ReadSignal<String>,
) -> Element {
    let api_client: Signal<ApiClient> = use_context();
    let mut build: BuildState = use_context();
    let mut project: Signal<Project> = use_context();
    let mut errors: Errors = use_context();
    let mut hot_reload: HotReload = use_context();

    let mut share_btn_text = use_signal(|| "Share");

    rsx! {
        div { id: "dxp-header",
            // Left pane header
            div {
                id: "dxp-header-left",
                style: if let Some(val) = pane_left_width() { "width:{val}px;" },

                // Examples button/menu
                Select::<Project> {
                    value: Some(project()),
                    on_value_change: move |example: Option<Project>| {
                        use crate::monaco;
                        let Some(example) = example else {
                            return;
                        };

                        project.set(example.clone());
                        build.set_stage(BuildStage::Finished(Ok(example.id())));
                        monaco::set_current_model_value(&example.contents());
                        hot_reload.set_starting_code(&example.contents());
                    },
                    SelectTrigger {
                        {file_name}
                    }
                    SelectList {
                        for (index, example) in example_projects::get_example_projects().iter().enumerate() {
                            SelectOption::<Project> {
                                index,
                                value: example.clone(),
                                text_value: example.path.clone(),
                                div {
                                    display: "flex",
                                    flex_direction: "column",
                                    align_items: "left",
                                    padding: "0.25rem",
                                    h3 {
                                        margin: "0",
                                        margin_bottom: ".25rem",
                                        {example.path.clone()}
                                    }
                                    p {
                                        margin: "0",
                                        {example.description.clone()}
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Right pane header
            div {
                id: "dxp-header-right",
                style: if let Some(val) = pane_right_width() { "width:{val}px;" } else { "".to_string() },

                // Share button
                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: move |_| async move {
                        share_btn_text.set("Sharing...");
                        match copy_share_link(&api_client(), project, urls.location).await {
                            Ok(()) => share_btn_text.set("Link Copied!"),
                            Err(error) => {
                                share_btn_text.set("Error!");
                                errors
                                    .push_error((
                                        "Share Failed",
                                        format!(
                                            "An error occured while generating a share link: {error:?}",
                                        ),
                                    ));
                            }
                        }
                    },
                    "{share_btn_text}"
                }


                // Run button
                Button {
                    variant: ButtonVariant::Outline,
                    "data-disabled": build.stage().is_running(),
                    display: "flex",
                    flex_direction: "row",
                    align_items: "center",
                    gap: "0.5rem",
                    onclick: move |_| {
                        on_rebuild.call(());
                    },

                    if build.stage().is_running() {
                        Progress {}
                    } else {
                        "Rebuild"
                    }

                }
            }
        }
    }
}

#[component]
fn Progress() -> Element {
    let build = use_context::<BuildState>();

    // Generate the loading message.
    let message = use_memo(move || {
        let compiling = build.stage().get_compiling_stage();
        if let Some((crates_compiled, total_crates, current_crate)) = compiling {
            return format!("{crates_compiled}/{total_crates}");
        }

        match build.stage() {
            BuildStage::NotStarted => "Waiting".to_string(),
            BuildStage::Starting => "Starting".to_string(),
            BuildStage::Waiting(time) => {
                format!("Waiting {}s", time.as_secs())
            }
            BuildStage::Building(build_stage) => match build_stage {
                model::BuildStage::RunningBindgen => "Binding".to_string(),
                model::BuildStage::Other => "Computing".to_string(),
                model::BuildStage::Compiling { .. } => unreachable!(),
            },
            BuildStage::Finished(_) => "Finished!".to_string(),
        }
    });

    rsx! {
        LoadingSpinner {}
        "{message}"
    }
}
