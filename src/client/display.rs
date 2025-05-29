// httpp - Nathanael "NateNateNate" Thevarajah
// <natenatenat3@protonmail.com> - Refer to the license for more
// information.

pub fn display_response(status: reqwest::StatusCode, body: &str) {
    println!("Status: {}", status);

    match serde_json::from_str::<serde_json::Value>(body) {
        Ok(json) => {
            println!(
                "Body:\n{}",
                serde_json::to_string_pretty(&json).unwrap_or_else(|_| body.to_string())
            );
        }
        Err(_) => {
            println!("Body:\n{}", body);
        }
    }
}
