use std::io;

use reqwest::blocking::Client;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";

enum RequestType {
    GET,
    POST,
}

fn main() {
    let url = match prompt_and_get_url() {
        Some(url) => url,
        None => {
            log("Failed to get URL, exiting", RED);
            return;
        }
    };

    let request_type = match prompt_and_get_request_type() {
        Some(rt) => rt,
        None => {
            log("Failed to get request type, exiting", RED);
            return;
        }
    };

    let data = prompt_for_data(&request_type).unwrap_or_default();

    let http_client = Client::new();
    let response = match request_type {
        RequestType::POST => http_client.post(url).body(data).send(),
        RequestType::GET => http_client.get(url).send(),
    };

    let response_text = match response {
        Ok(res) => res
            .text()
            .unwrap_or_else(|_| "cannot read response".to_string()),
        Err(_) => "Error sending request".to_string(),
    };

    let msg = format!("Response: {}", response_text);
    log(&msg, GREEN);
}

fn prompt_and_get_url() -> Option<String> {
    let mut url = String::new();
    log("Please enter the url:", BLUE);
    io::stdin().read_line(&mut url).ok()?;
    let url = url.trim();
    Some(url.to_string())
}

fn prompt_and_get_request_type() -> Option<RequestType> {
    loop {
        log("Please select the request type: \n1. POST \n2. GET", BLUE);
        let mut request_type = String::new();
        io::stdin().read_line(&mut request_type).ok()?;

        let request_type: u32 = match request_type.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                log("Unable to parse, please type a number", RED);
                continue;
            }
        };

        match request_type {
            1 => return Some(RequestType::POST),
            2 => return Some(RequestType::GET),
            _ => log("Invalid choice, try again!", RED),
        }
    }
}

fn prompt_for_data(request_type: &RequestType) -> Option<String> {
    let mut data = String::new();
    match request_type {
        RequestType::POST => {
            log("Enter POST data", BLUE);
            io::stdin().read_line(&mut data).ok()?;
            Some(data.to_string())
        }
        RequestType::GET => Some(String::new()),
    }
}

fn log(message: &str, color: &str) {
    eprintln!("{}{message}{}", color, RESET);
}
