// ANCHOR: GotchasApp
#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let mut amount = use_signal(|| 100.0);
    let mut currency = use_signal(|| "USD".to_string());

    rsx! {
        div {
            h1 { "Currency Converter" }

            div {
                label { "Amount: " }
                input {
                    r#type: "number",
                    value: "{amount()}",
                    oninput: move |evt| {
                        if let Ok(val) = evt.value().parse::<f64>() {
                            amount.set(val);
                        }
                    }
                }
            }

            div {
                label { "Currency: " }
                select {
                    onchange: move |evt| {
                        currency.set(evt.value());
                    },
                    option { value: "USD", selected: currency() == "USD", "USD" }
                    option { value: "EUR", selected: currency() == "EUR", "EUR" }
                    option { value: "JPY", selected: currency() == "JPY", "JPY" }
                }
            }

            CurrencyConverter {
                amount: amount(),
                currency: currency(),
            }
        }
    }
}

#[component]
fn CurrencyConverter(amount: ReadOnlySignal<f64>, currency: ReadOnlySignal<String>) -> Element {
    let exchange_rate = use_resource(move || {
        let amount = amount();
        let currency = currency();
        async move {
            let rate = fetch_rate(&currency).await;
            amount * rate
        }
    });

    rsx! {
        div {
            match exchange_rate() {
                Some(value) => rsx!("Converted: ${value:.2}"),
                None => rsx!("Fetching rate..."),
            }
        }
    }
}

async fn fetch_rate(currency: &str) -> f64 {
    // API call
    match currency {
        "USD" => 1.0,
        "EUR" => 0.91,
        _ => 1.0,
    }
}
// ANCHOR_END: GotchasApp

// ANCHOR: Temperature
#[component]
fn Temperature(celsius: f64) -> Element {
    let fahrenheit = use_memo(move || (celsius * 9.0 / 5.0) + 32.0);

    rsx! {
        div {
            "Temperature: {celsius}°C / {fahrenheit:.2}°F"
        }
    }
}
// ANCHOR_END: Temperature

// ANCHOR: ReactiveTemperature
#[component]
fn ReactiveTemperature(celsius: ReadOnlySignal<f64>) -> Element {
    let fahrenheit = use_memo(move || (celsius() * 9.0 / 5.0) + 32.0);

    rsx! {
        div {
            "Temperature: {celsius}°C / {fahrenheit:.2}°F"
        }
    }
}
// ANCHOR_END: ReactiveTemperature

// ANCHOR: UseReactive
#[component]
fn Investment(principal: f64, rate: f64) -> Element {
    let doubled = use_memo(use_reactive!(|principal, rate| principal * (1.0 + rate)));

    rsx! {
        div {
            "Projected Balance: ${doubled:.2}"
        }
    }
}
// ANCHOR_END: UseReactive

// ANCHOR: UseReactiveGotcha
#[component]
fn Report(metric: ReadOnlySignal<i32>, unit: String) -> Element {
    let summary = use_memo(use_reactive!(|unit| format!("{} {}", metric(), unit)));
    rsx! { div { "{summary}" } }
}
// ANCHOR_END: UseReactiveGotcha

// ANCHOR: UseFutureGotcha
#[component]
fn Bad(query: String) -> Element {
    use_future(move || {
        let value = query.clone();
        async move { fetch_rate(&value).await }
    });

    rsx! {}
}
// ANCHOR_END: UseFutureGotcha

// ANCHOR: UseFutureCorrect
#[component]
fn Good(query: ReadOnlySignal<String>) -> Element {
    let data = use_resource(move || async move { fetch_rate(&query()).await });
    rsx! {
       div {
           match data() {
               Some(value) => rsx!("{value}"),
               None => rsx!("Fetching rate..."),
           }
       }
    }
}
// ANCHOR_END: UseFutureCorrect
