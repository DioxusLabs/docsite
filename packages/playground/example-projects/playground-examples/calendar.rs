//! A simple example showcasing the dx-components library.

mod components;

use components::calendar::*;
use dioxus::prelude::*;
use time::{macros::date, Date, UtcDateTime};

static THEME: Asset = asset!("/assets/dx-components-theme.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);
    let mut view_date = use_signal(|| UtcDateTime::now().date());
    rsx! {
        document::Stylesheet { href: THEME }
        div {
            display: "flex",
            align_items: "center",
            justify_content: "center",
            height: "100vh",
            width: "100vw",
            div {
                width: "258px",
                Calendar {
                    selected_date: selected_date(),
                    on_date_change: move |date| {
                        selected_date.set(date);
                    },
                    view_date: view_date(),
                    on_view_change: move |new_view: Date| {
                        view_date.set(new_view);
                    },
                    min_date: date!(1995 - 07 - 21),
                    max_date: date!(2035 - 09 - 11),
                    CalendarHeader {
                        CalendarNavigation {
                            CalendarPreviousMonthButton {}
                            CalendarSelectMonth {}
                            CalendarSelectYear {}
                            CalendarNextMonthButton {}
                        }
                    }
                    CalendarGrid {}
                }
            }
        }
    }
}
