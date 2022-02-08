use std::collections::HashMap;

use std::io::Result;

pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
}

pub struct HttpMetadata {
    version: String,
    resource_path: String,
    headers: HashMap<String, String>,
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub metadata: HttpMetadata,
    pub content: String,
}

pub fn parse_http_request(request: &[u8]) -> Result<HttpRequest> {
    // turn bytes into a string
    let request_str = std::str::from_utf8(request).unwrap();
    let mut lines = request_str.lines();

    // read request-line
    let request_line = lines.next().unwrap();
    let request_line: Vec<&str> = request_line.split(' ').collect();

    if request_line.len() != 3 {
        panic!("resource line could not be parsed");
    }

    let method = request_line[0];
    let request_method: Option<HttpMethod>;
    if method == "GET" {
        request_method = Some(HttpMethod::GET);
    } else if method == "HEAD" {
        request_method = Some(HttpMethod::HEAD);
    } else if method == "POST" {
        request_method = Some(HttpMethod::POST);
    } else if method == "PUT" {
        request_method = Some(HttpMethod::PUT);
    } else if method == "DELETE" {
        request_method = Some(HttpMethod::DELETE);
    } else {
        panic!("unexpected http method");
    }

    let resource_path = String::from(request_line[1]);
    let version = String::from(request_line[2]);

    // read headers (case-insensitive)
    let mut request_headers = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        if let Some(pos) = line.find(':') {
            let (key, value) = line.split_at(pos);
            request_headers.insert(key.to_lowercase(), String::from(value));
        }
    }

    let metadata = HttpMetadata {
        version,
        resource_path,
        headers: request_headers,
    };
    // read content
    let content = lines.collect::<Vec<&str>>().join("\n");
    let content = content.trim_matches(char::from(0)).to_string();

    Ok(HttpRequest {
        method: request_method.unwrap(),
        metadata,
        content,
    })
}
