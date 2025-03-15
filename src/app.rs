use chrono::{NaiveDate, Utc};
use dioxus::prelude::*;

// Parse URL parameters from either query string or hash fragment
fn parse_params(url_str: &str) -> Vec<(String, String)> {
    let params_str = url_str
        .split('?')
        .nth(1)
        .or_else(|| url_str.split('#').nth(1));

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

// Helper function to extract a parameter value with a default
fn get_param<T, F>(params: &[(String, String)], key: &str, parser: F, default: T) -> T
where
    F: FnOnce(&str) -> Option<T>,
{
    params
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| parser(v))
        .unwrap_or(default)
}

// Display a label-value row
fn info_row<'a>(label: &'a str, value: String) -> LazyNodes<'a, 'a> {
    rsx! {
        div {
            class: "flex justify-between mb-3",
            label { class: "font-semibold", "{label}" }
            span { "{value}" }
        }
    }
}

// Create an example link with parameters
fn example_link<'a>(
    amount: f64,
    interest: f64,
    start_date: &'a str,
    description: &'a str,
    extra_params: &'a str,
) -> LazyNodes<'a, 'a> {
    let extra = if extra_params.is_empty() {
        String::new()
    } else {
        format!("&{}", extra_params)
    };

    let params = format!(
        "?amount={}&interest={}&start_date={}{}",
        amount, interest, start_date, extra
    );

    rsx! {
        a {
            class: "text-blue-300 hover:underline text-center",
            href: "{params}",
            "{description}"
        }
    }
}

// Get compound frequency description
fn compound_description(freq: u32) -> &'static str {
    match freq {
        1 => "Annual",
        2 => "Semi-annual",
        4 => "Quarterly",
        12 => "Monthly",
        365 => "Daily",
        _ => "Custom",
    }
}

// Calculate compound interest
fn calculate_compound_interest(
    amount: f64,
    interest_rate: f64,
    years: f64,
    compound_freq: u32,
) -> f64 {
    if compound_freq > 1 {
        // Compound interest formula: P(1 + r/n)^(nt)
        amount * (1.0 + interest_rate / compound_freq as f64).powf(compound_freq as f64 * years)
    } else {
        // Simple annual compounding
        amount * (1.0 + interest_rate).powf(years)
    }
}

pub fn app(cx: Scope) -> Element {
    // Get URL parameters
    let window = web_sys::window().unwrap();
    let href = window.location().href().unwrap();
    let params = parse_params(&href);

    // Extract parameters
    let amount = get_param(&params, "amount", |v| v.parse::<f64>().ok(), 0.0);
    let interest = get_param(&params, "interest", |v| v.parse::<f64>().ok(), 0.0);
    let start_date = get_param(
        &params,
        "start_date",
        |v| NaiveDate::parse_from_str(v, "%Y-%m-%d").ok(),
        Utc::now().date_naive(),
    );
    let compound_freq = get_param(&params, "compound", |v| v.parse::<u32>().ok(), 1);

    // Calculate values
    let days_passed = (Utc::now().date_naive() - start_date).num_days();
    let years_passed = days_passed as f64 / 365.0;
    let interest_rate = interest / 100.0;
    let current_value =
        calculate_compound_interest(amount, interest_rate, years_passed, compound_freq);

    // Format display strings
    let amount_str = format!("${:.2}", amount);
    let interest_str = format!("{}%", interest);
    let date_str = start_date.format("%Y-%m-%d").to_string();
    let days_str = days_passed.to_string();
    let current_value_str = format!("${:.2}", current_value);
    let compound_text = compound_description(compound_freq);

    // Render UI
    cx.render(rsx! {
        div {
            class: "max-w-lg mx-auto p-6",

            h1 {
                class: "text-2xl font-bold mb-6 text-center",
                "Debt Value Calculator"
            }

            // Main calculator card
            div {
                class: "bg-gray-900 rounded-lg shadow p-6",

                info_row("Initial Amount:", amount_str)
                info_row("Annual Interest Rate:", interest_str)
                info_row("Start Date:", date_str)
                info_row("Days Passed:", days_str)
                info_row("Compounding:", compound_text.to_string())

                div {
                    class: "mt-6 pt-4 border-t border-gray-700 flex justify-between",
                    label { class: "text-xl font-bold", "Current Value:" }
                    span {
                        class: "text-xl font-bold result",
                        "{current_value_str}"
                    }
                }
            }

            // Help and examples section
            div {
                class: "mt-6 text-sm text-gray-300",
                p {
                    class: "text-center mb-2",
                    "Add parameters to the URL: ?amount=1000&interest=5&start_date=2023-01-01&compound=12"
                }

                div {
                    class: "mt-4 p-4 bg-gray-800 rounded-lg",
                    h3 {
                        class: "font-bold mb-2 text-center",
                        "Examples:"
                    }
                    div {
                        class: "grid grid-cols-1 gap-2",
                        example_link(1000.0, 5.0, "2023-01-01", "$1,000 at 5% from 2023-01-01", "")
                        example_link(5000.0, 3.5, "2024-01-01", "$5,000 at 3.5% from 2024-01-01", "")
                        example_link(10000.0, 7.0, "2022-06-15", "$10,000 at 7% (monthly compounding)", "compound=12")
                        example_link(25000.0, 4.25, "2020-03-01", "$25,000 at 4.25%", "")
                    }
                }
            }
        }
    })
}
