use std::io;

use reqwest::blocking::Client;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";

enum RequestType {
    GET,
    POST
}

fn main() {
    let url = prompt_and_get_url();
    let request_type =  prompt_and_get_request_type();

    let data = match prompt_for_data(&request_type) {
        Ok(data) => data,
        Err(_) => String::new()
    };

    let http_client = Client::new();
    let response = match request_type {
        RequestType::POST => http_client.post(url).body(data).send(),
        RequestType::GET => http_client.get(url).send()
    };

    let response_text = match response {
        Ok(res) => res.text().unwrap_or_else(|_| "cannot read response".to_string()),
    Err(_) => "Error sending request".to_string(),
    };

    let msg = format!("Response: {}", response_text);
    log(&msg, GREEN);

}



fn prompt_and_get_url() -> String {
    let mut url = String::new();
    log("Please enter the url:", BLUE);
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");
    let url = url.trim();
    url.to_string()
}

fn prompt_and_get_request_type() -> RequestType {
    loop {
        log("Please select the request type: \n1. POST \n2. GET", BLUE);
        let mut request_type = String::new();
        io::stdin()
            .read_line(&mut request_type)    
            .expect("Cannot read line");

        let request_type: u32 = request_type.trim().parse().expect("Unable to parse, please type a nubmer");

        match request_type {
            1 => return RequestType::POST,
            2 => return RequestType::GET,
            _ => log("Invalid choice, try again!", RED)
        }
    }

}

fn prompt_for_data(request_type: &RequestType) -> Result<String, String> {
    let mut data = String::new();
    match request_type {
        RequestType::POST => {
            log("Enter POST data", BLUE);
            io::stdin()
                .read_line(&mut data)    
                .expect("Cannot read line");
            Ok(data.to_string())
        },
        RequestType::GET => {
            Ok(String::new())
        }
    }
}

fn log(message: &str, color: &str) {
    eprintln!("{}{message}{}", color, RESET);
}