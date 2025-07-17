use dioxus::prelude::*;

mod throw_error {
    use super::*;
    // ANCHOR: throw_error
    #[component]
    fn ThrowsError() -> Element {
        // You can return any type that implements `Error`
        let number: i32 = use_hook(|| "1.234").parse()?;

        todo!()
    }
    // ANCHOR_END: throw_error
}

mod capture_error {
    use super::*;
    // ANCHOR: capture_error
    #[component]
    fn Parent() -> Element {
        rsx! {
            ErrorBoundary {
                // The error boundary accepts a closure that will be rendered when an error is thrown in any
                // of the children
                handle_error: |_| {
                    rsx! { "Oops, we encountered an error. Please report this to the developer of this application" }
                },
                ThrowsError {}
            }
        }
    }
    // ANCHOR_END: capture_error

    #[component]
    fn ThrowsError() -> Element {
        let number: i32 = use_hook(|| "1...234").parse()?;

        todo!()
    }
}

mod throw_error_event_handler {
    use super::*;
    // ANCHOR: throw_error_event_handler
    #[component]
    fn ThrowsError() -> Element {
        rsx! {
            button {
                onclick: move |_| {
                    // Event handlers can return errors just like components
                    let number: i32 = "1...234".parse()?;

                    tracing::info!("Parsed number: {number}");

                    Ok(())
                },
                "Throw error"
            }
        }
    }
    // ANCHOR_END: throw_error_event_handler
}

mod add_context {
    use super::*;
    // ANCHOR: add_context
    #[component]
    fn ThrowsError() -> Element {
        // You can call the context method on results to add more information to the error
        let number: i32 = use_hook(|| "1.234")
            .parse()
            .context("Failed to parse name")?;

        todo!()
    }
    // ANCHOR_END: add_context
}

mod show {
    use super::*;
    // ANCHOR: show
    #[component]
    fn Parent() -> Element {
        rsx! {
            ErrorBoundary {
                // The error boundary accepts a closure that will be rendered when an error is thrown in any
                // of the children
                handle_error: |error: ErrorContext| {
                    if let Some(error_ui) = error.show() {
                        rsx! {
                            {error_ui}
                        }
                    } else {
                        rsx! {
                            div {
                                "Oops, we encountered an error. Please report this to the developer of this application"
                            }
                        }
                    }
                },
                ThrowsError {}
            }
        }
    }
    // ANCHOR_END: show

    #[component]
    fn ThrowsError() -> Element {
        let number: i32 = use_hook(|| "1...234").parse().show(|error| {
            rsx! {
                div {
                    background_color: "red",
                    color: "white",
                    "Error parsing number: {error}"
                }
            }
        })?;

        todo!()
    }
}

pub use phone_number_validation::PhoneNumberValidation;

mod phone_number_validation {
    use super::*;
    use std::convert::TryInto;
    use std::str::FromStr;

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    struct PhoneNumber {
        country_code: Option<u32>,
        sections: [u32; 3],
    }

    impl std::fmt::Display for PhoneNumber {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(country_code) = self.country_code {
                write!(f, "+{country_code} ")?;
            }
            let [area_code, exchange, subscriber] = &self.sections;
            write!(f, "{}-{}-{}", area_code, exchange, subscriber)
        }
    }

    impl FromStr for PhoneNumber {
        type Err = String;

        fn from_str(phone_number: &str) -> Result<Self, Self::Err> {
            let phone_number = phone_number.trim();
            let mut sections = phone_number.split('-');
            let country_code = if phone_number.starts_with('+') {
                match sections.next() {
                    Some(country_code) => Some(
                        country_code
                            .parse::<u32>()
                            .map_err(|_| "Invalid country code".to_string())?,
                    ),
                    None => return Err("Expected country code after +".to_string()),
                }
            } else {
                None
            };

            let sections = sections
                .map(|s| s.parse::<u32>())
                .collect::<Result<Box<[_]>, _>>()
                .map_err(|_| "failed to parse phone number")?;
            if sections.len() != 3 {
                return Err("Expected 3 numbers after the country code".to_string());
            }
            let mut sections_array = [0; 3];
            sections_array
                .iter_mut()
                .zip(sections)
                .for_each(|(a, b)| *a = b);

            Ok(PhoneNumber {
                country_code,
                sections: sections_array,
            })
        }
    }

    // ANCHOR: phone_number_validation
    #[component]
    pub fn PhoneNumberValidation() -> Element {
        let mut phone_number = use_signal(|| String::new());
        let parsed_phone_number = use_memo(move || phone_number().parse::<PhoneNumber>());

        rsx! {
            input {
                class: "border border-gray-300 rounded-md p-2 mb-4",
                placeholder: "Phone number",
                value: "{phone_number}",
                oninput: move |e| {
                    phone_number.set(e.value());
                },
            }

            match parsed_phone_number() {
                Ok(phone_number) => rsx! {
                    div {
                        "Parsed phone number: {phone_number}"
                    }
                },
                Err(error) => rsx! {
                    div {
                        "Phone number is invalid: {error}"
                    }
                }
            }
        }
    }
    // ANCHOR_END: phone_number_validation
}
