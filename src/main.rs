use dotenv::dotenv;
use reqwest::{blocking::Client, Error, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct SMSResponse {
    account_sid: Option<String>,
    api_version: String,
    body: String,
    date_created: String,
    date_sent: String,
    date_updated: String,
    direction: String,
    error_code: String,
    error_message: String,
    from: String,
    messaging_service_sid: String,
    num_media: String,
    num_segments: String,
    price: String,
    price_unit: String,
    sid: String,
    status: String,
    subresource_uris: SubresourceUris,
    to: String,
    uri: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    more_info: String,
    status: u16,
}

#[derive(Serialize, Deserialize)]
struct SubresourceUris {
    all_time: String,
    today: String,
    yesterday: String,
    this_month: String,
    last_month: String,
    daily: String,
    monthly: String,
    yearly: String,
}

fn handle_error(body: String) {
    let error_response: ErrorResponse =
        serde_json::from_str(&body).expect("Unable to deserialise JSON error response.");
    println!(
        "SMS was not able to be sent because: {:?}.",
        error_response.message
    );
}

fn handle_success(body: String) {
    let sms_response: SMSResponse =
        serde_json::from_str(&body).expect("Unable to deserialise JSON success response.");
    println!("Your SMS with the body \"{:?}\".", sms_response.body);
}

fn main() -> Result<(), Error> {
    dotenv().ok();

    let twilio_account_sid =
        env::var("TWILIO_ACCOUNT_SID").expect("Twilio Account SID could not be retrieved.");
    let twilio_auth_token =
        env::var("TWILIO_AUTH_TOKEN").expect("Twilio Auth Token could not be retrieved.");
    let twilio_phone_number =
        env::var("TWILIO_PHONE_NUMBER").expect("Twilio Auth Token could not be retrieved.");
    let recipient_phone_number =
        env::var("RECIPIENT_PHONE_NUMBER").expect("Twilio Auth Token could not be retrieved.");

    let sms_body = "G'day from Rust and Twilio".to_string();

    let request_url =
        format!("https://api.twilio.com/2010-04-01/Accounts/{twilio_account_sid}/Messages.json");

    let client = Client::new();
    let request_params = [
        ("to", &recipient_phone_number),
        ("from", &twilio_phone_number),
        ("body", &sms_body),
    ];
    let response = client
        .post(&request_url)
        .basic_auth(twilio_account_sid, Some(twilio_auth_token))
        .form(&request_params)
        .send()?;

    let status = response.status();
    let body_result = response.text();
    let body = match body_result {
        Ok(result) => result,
        Err(error) => panic!(
            "Problem extracting the JSON body content. Reason: {:?}",
            error
        ),
    };

    match status {
        StatusCode::BAD_REQUEST => handle_error(body),
        StatusCode::OK => handle_success(body),
        _ => panic!("Unknown error code"),
    }

    Ok(())
}
