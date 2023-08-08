# Send an SMS with Rust and Twilio

This is a small project showing how to send an SMS with Twilio using Rust, and forms the basis of an upcoming [Twilio tutorial](https://twilio.com/blog).

## Prerequisites

To use the code, you will need the following:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Usage

To get started with the project, create a file named .env in the project's top-level directory. 
Then, add the following configuration to it:

```ini
TWILIO_AUTH_TOKEN="<<TWILIO_AUTH_TOKEN>>"
TWILIO_ACCOUNT_SID="<<TWILIO_ACCOUNT_SID>>"
TWILIO_PHONE_NUMBER="<<TWILIO_PHONE_NUMBER>>"
RECIPIENT_PHONE_NUMBER="<<RECIPIENT_PHONE_NUMBER>>"
```

After that, replace the placeholders with your Twilio Auth Token, Account SID, and phone number, which you can find in [the Twilio Console Dashboard](https://console.twilio.com).
Then, add an [E.164](https://www.twilio.com/docs/glossary/what-e164) formatted phone number for the recipient.

With that, the code is ready to use. 
So, in the terminal, in the project's top-level directory, run the following command to run the code and send an SMS:

```bash
cargo run
```
