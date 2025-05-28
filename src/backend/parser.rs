// httpp - Nathanael "NateNateNate" Thevarajah
// <natenatenat3@protonmail.com> - Refer to the license for more
// information.

use crate::backend::lexer::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<std::collections::HashMap<String, String>>,
}

enum Body {
    Json(HashMap<String, String>),
}

pub fn interpolate(s: &str, ctx: &std::collections::HashMap<String, String>) -> String {
    let mut out = s.to_string();
    for (key, value) in ctx {
        let placeholder = format!("{{{{{}}}}}", key);
        out = out.replace(&placeholder, value);
    }
    out
}

pub fn anal(tokens: &[Token]) -> Result<Request, String> {
    let mut index = 0;

    macro_rules! next {
        () => {{
            let t = tokens.get(index);
            index += 1;
            t
        }};
    }

    let method = match next!() {
        Some(Token::Method(m)) => m.clone(),
        other => return Err(format!("Expected method, got {:?}", other)),
    };

    let path = match next!() {
        Some(Token::Path(p)) => p.clone(),
        other => return Err(format!("Expected path, got {:?}", other)),
    };

    let mut headers = Vec::new();
    while let (Some(Token::HeaderKey(k)), Some(Token::HeaderValue(v))) =
        (tokens.get(index), tokens.get(index + 1))
    {
        headers.push((k.clone(), v.clone()));
        index += 2;
    }

    if matches!(tokens.get(index), Some(Token::Newline)) {
        index += 1;
    }

    let mut body = None;
    if matches!(tokens.get(index), Some(Token::LeftBrace)) {
        index += 1;
        let mut map = std::collections::HashMap::new();

        while let (Some(Token::HeaderKey(k)), Some(Token::HeaderValue(v))) =
            (tokens.get(index), tokens.get(index + 1))
        {
            let key = k.trim_matches('"').to_string();
            let val = v.trim_matches(['"', ','].as_ref()).to_string();
            map.insert(key, val);
            index += 2;
        }

        if matches!(tokens.get(index), Some(Token::RightBrace)) {
            index += 1;
        } else {
            return Err("Missing right brace in body".into());
        }

        body = Some(map);
    }

    Ok(Request {
        method,
        path,
        headers,
        body,
    })
}
