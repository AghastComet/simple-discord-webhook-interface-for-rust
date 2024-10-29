use std::collections::HashMap;
use reqwest::blocking::multipart::Part;

#[derive(Debug)]
pub enum WebhookError{
    RequestError(reqwest::Error),
    IOError(std::io::Error),
}
impl From<reqwest::Error> for WebhookError{
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}
impl From<std::io::Error> for WebhookError{
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

/// This will send a message to the webhook.
/// This does not have any special features like setting a custom username at this time.
pub fn send_text(text: &str, webhook_url: &str) -> Result<reqwest::blocking::Response, WebhookError>{
    let args = HashMap::from([("content", text)]);
    let client = reqwest::blocking::Client::new();
    Ok(client
        .post(webhook_url)
        .json(&args)
        .send()?)
}

/// This sends an image to the webhook.
/// A filename is required due to how discord works, but the value does not matter, including the file extension.
/// From testing, this worked when image_data was bytes pulled directly from a PNG file or JPEG file.
pub fn send_image_from_bytes(filename: &str, image_data: &[u8], webhook_url: &str) -> Result<reqwest::blocking::Response, WebhookError>{
    let form = reqwest::blocking::multipart::Form::new()
        .part(filename.to_string(), Part::bytes(Vec::from(image_data)).file_name(filename.to_string()));
       
    Ok(reqwest::blocking::Client::new()
        .post(webhook_url)
        .multipart(form)
        .send()?)
}

/// This sends an image from disk to the webhook.
/// This isn't as tested as the other two, but it did work when writing.
pub fn send_image_from_file(filename: &str, path: &str, webhook_url: &str) -> Result<reqwest::blocking::Response, WebhookError>{
    let form = reqwest::blocking::multipart::Form::new()
        .file(filename.to_string(), path)?;

       
    Ok(reqwest::blocking::Client::new()
        .post(webhook_url)
        .multipart(form)
        .send()?)
}
