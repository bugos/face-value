use chrono::{NaiveDate, Utc};
use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(app);
}

fn parse_params(url_str: &str) -> Vec<(String, String)> {
    let params_str = if url_str.contains('?') {
        url_str.split('?').nth(1)
    } else if url_str.contains('#') {
        url_str.split('#').nth(1)
    } else {
        None
    };

    params_str
        .map(|s| {
            s.split('&')
                .filter_map(|pair| {
                    let mut parts = pair.split('=');
                    match (parts.next(), parts.next()) {
                        (Some(k), Some(v)) => Some((k.to_string(), v.to_string())),
                        _ => None,
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

fn info_row<'a>(label: &'a str, value: String, class: &'a str) -> LazyNodes<'a, 'a> {
    rsx! {
        div {
            class: "{class}",
            label { class: "font-semibold", "{label}" }
            span { "{value}" }
        }
    }
}

fn app(cx: Scope) -> Element {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let href = location.href().unwrap();
    let params = parse_params(&href);

    let amount = params
        .iter()
        .find(|(k, _)| k == "amount")
        .and_then(|(_, v)| v.parse::<f64>().ok())
        .unwrap_or(0.0);

    let interest = params
        .iter()
        .find(|(k, _)| k == "interest")
        .and_then(|(_, v)| v.parse::<f64>().ok())
        .unwrap_or(0.0);

    let start_date = params
        .iter()
        .find(|(k, _)| k == "start_date")
        .and_then(|(_, v)| NaiveDate::parse_from_str(v, "%Y-%m-%d").ok())
        .unwrap_or_else(|| Utc::now().date_naive());

    let days_passed = (Utc::now().date_naive() - start_date).num_days();
    let interest_factor = 1.0 + (interest / 100.0);
    let current_value = amount * interest_factor.powf(days_passed as f64 / 365.0);

    let amount_str = format!("${:.2}", amount);
    let interest_str = format!("{}%", interest);
    let date_str = start_date.format("%Y-%m-%d").to_string();
    let days_str = days_passed.to_string();
    let current_value_str = format!("${:.2}", current_value);

    cx.render(rsx! {
        div {
            class: "container mx-auto p-4 max-w-md",
            style: "font-family: Arial, sans-serif;",

            h1 {
                class: "text-2xl font-bold mb-4 text-center",
                "Debt Value Calculator"
            }

            div {
                class: "bg-white rounded-lg shadow-md p-6",

                info_row("Initial Amount: ", amount_str, "mb-4")
                info_row("Annual Interest Rate: ", interest_str, "mb-4")
                info_row("Start Date: ", date_str, "mb-4")
                info_row("Days Passed: ", days_str, "mb-4")

                div {
                    class: "mt-6 pt-4 border-t",
                    label { class: "text-xl font-bold", "Current Value: " }
                    span {
                        class: "text-xl text-green-600",
                        "{current_value_str}"
                    }
                }
            }

            p {
                class: "mt-4 text-sm text-gray-600 text-center",
                "Parameters can be added using either URL parameters (?amount=1000&interest=5) or hash (#amount=1000&interest=5)"
            }
            p {
                class: "text-sm text-gray-600 text-center",
                "Example: index.html#amount=1000&interest=5&start_date=2023-01-01"
            }
        }
    })
}
