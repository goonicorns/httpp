// httpp.rs - Nathanael "NateNateNate" Thevarajah
// <natenatenat3@protonmail.com> - Refer to the license for more
// information.

use crate::backend::parser::Request;

pub async fn send_request(req: Request) -> (reqwest::StatusCode, String) {
    let client = reqwest::Client::new();

    let mut request_builder = client.request(req.method.parse().unwrap(), &req.path);

    for (k, v) in &req.headers {
        request_builder = request_builder.header(k, v);
    }

    if let Some(body) = &req.body {
        request_builder = request_builder.json(&body);
    }

    let response = request_builder
        .send()
        .await
        .expect("Failed to send request");

    let status = response.status();
    let text = response.text().await.expect("Failed to read response body");

    (status, text)
}
