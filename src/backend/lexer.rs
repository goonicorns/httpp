// httpp - Nathanael "NateNateNate" Thevarajah
// <natenatenat3@protonmail.com> - Refer to the license for more
// information.

#[derive(Debug, PartialEq)]
pub enum Token {
    Method(String),
    Path(String),
    HeaderKey(String),
    HeaderValue(String),
    AcceptKey(String),
    AcceptValue(String),
    LeftBrace,
    RightBrace,
    Interpolation(String),
    StringLiteral(String),
    Newline,
    Unknown(String),
}

pub fn lex_httpp(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            tokens.push(Token::Newline);
            continue;
        }

        if let Some((method, path)) = line.split_once(' ') {
            if ["GET", "POST", "PUT", "PATCH", "DELETE"].contains(&method) {
                tokens.push(Token::Method(method.to_string()));
                tokens.push(Token::Path(path.to_string()));
                continue;
            }
        }

        if let Some((key, value)) = line.split_once(':') {
            tokens.push(Token::HeaderKey(key.trim().to_string()));
            tokens.push(Token::HeaderValue(value.trim().to_string()));
            continue;
        }

        if line == "{" {
            tokens.push(Token::LeftBrace);
        } else if line == "}" {
            tokens.push(Token::RightBrace);
        } else {
            if line.contains("{{") && line.contains("}}") {
                let start = line.find("{{").unwrap();
                let end = line.find("}}").unwrap() + 2;
                let interp = &line[start..end];
                tokens.push(Token::Interpolation(interp.to_string()));
            } else {
                tokens.push(Token::StringLiteral(line.to_string()));
            }
        }
    }

    tokens
}
