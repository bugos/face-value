use chrono::{NaiveDate, Utc};
use dioxus::prelude::*;

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
            class: "{class} flex justify-between mb-3",
            label { class: "font-semibold", "{label}" }
            span { "{value}" }
        }
    }
}

// Helper function to create example links with consistent behavior
fn example_link<'a>(
    amount: f64,
    interest: f64,
    start_date: &'a str,
    description: &'a str,
) -> LazyNodes<'a, 'a> {
    let params = format!(
        "amount={}&interest={}&start_date={}",
        amount, interest, start_date
    );

    rsx! {
        a {
            class: "text-blue-600 hover:underline text-center",
            href: "#{params}",
            "{description}"
        }
    }
}

pub fn app(cx: Scope) -> Element {
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
            class: "container",

            h1 {
                class: "text-2xl font-bold mb-6 text-center",
                "Debt Value Calculator"
            }

            div {
                class: "card",

                info_row("Initial Amount:", amount_str, "")
                info_row("Annual Interest Rate:", interest_str, "")
                info_row("Start Date:", date_str, "")
                info_row("Days Passed:", days_str, "")

                div {
                    class: "mt-6 pt-4 border-t flex justify-between",
                    label { class: "text-xl font-bold", "Current Value:" }
                    span {
                        class: "text-xl result",
                        "{current_value_str}"
                    }
                }
            }

            div {
                class: "mt-6 text-sm text-gray-600",
                p {
                    class: "text-center mb-2",
                    "Parameters can be added using either URL parameters (?amount=1000&interest=5) or hash (#amount=1000&interest=5)"
                }

                div {
                    class: "mt-4 p-4 bg-gray-100 rounded-lg",
                    h3 {
                        class: "font-bold mb-2 text-center",
                        "Examples:"
                    }
                    div {
                        class: "grid grid-cols-1 gap-2",
                        example_link(1000.0, 5.0, "2023-01-01", "$1,000 at 5% from 2023-01-01")
                        example_link(5000.0, 3.5, "2024-01-01", "$5,000 at 3.5% from 2024-01-01")
                        example_link(10000.0, 7.0, "2022-06-15", "$10,000 at 7% from 2022-06-15")
                        example_link(25000.0, 4.25, "2020-03-01", "$25,000 at 4.25% from 2020-03-01")
                    }
                }
            }
        }
    })
}
