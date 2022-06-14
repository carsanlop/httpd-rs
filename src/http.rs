use std::error::Error;
use std::str;

pub struct Request<'a> {
    pub method: &'a str,
    pub resource: &'a str,
    pub protocol: &'a str,
    pub headers: Vec<&'a str>
}

impl<'a> Request<'a> {
    pub fn new(buffer: &'a [u8]) -> Option<Request> {
        let lines = split(buffer);
    
        let mut method = "";
        let mut resource = "";
        let mut protocol = "";
        let mut headers = Vec::new();
    
        let mut section: u8 = 0;
        for s in lines {
            // Top line of the request.
            if section == 0 {
                (method, resource, protocol) = match parse_top(&s) {
                    Ok(v) => v,
                    Err(_) => return None,
                };

                section += 1;
            }
    
            // Headers
            else if section == 1 {
                if s == "\r\n" {
                    section += 1;
                }
                else {
                    headers.push(s)
                }
            }

            // Body
            else if section == 2 {
                // Body
            }
        }

        Some(Request {
            method: method,
            resource: resource,
            protocol: protocol,
            headers: headers
        })
    }


}

fn split<'a>(buffer: &'a [u8]) -> Vec<&'a str> {
    let mut lines = Vec::new();

    let len = buffer.len();
    let mut i: usize = 0;
    let mut start: usize = 0;
    let mut is_str: bool = false;
    loop {
        // Complete any pending line and exit.
        if i >= len {
            if is_str {
                lines.push(str::from_utf8(&buffer[start..i]).unwrap());
            }
            break;
        }

        let b = buffer[i];
        match b {
            
            b'\r' | b'\n' => {
                if is_str {
                    is_str = false;

                    lines.push(str::from_utf8(&buffer[start..i]).unwrap());
                }
            }
            _ => {
                if !is_str {
                    is_str = true;
                    start = i;
                }
            }
        }

        i += 1;
    }

    lines
}

fn parse_top(s: &str) -> Result<(&str, &str, &str), Box<dyn Error>> {
    if s.is_empty() {
        return Err("Empty request line".into());
    }

    if s.len() > 1024 {
        return Err("Too big line".into());
    }

    let mut tokens = s.split_whitespace();
    let method = match tokens.next() { Some(v) => v, None => return Err("Cannot parse method".into())};
    let resource = match tokens.next() { Some(v) => v, None => return Err("Cannot parse resource".into())};
    let protocol = match tokens.next() { Some(v) => v, None => return Err("Cannot parse protocol".into())};

    Ok((method, resource, protocol))
}


pub struct Response<'a> {
    pub status_code: u32,
    pub status_reason: &'a str,
    pub content: &'a str,
}

impl<'a> Response<'a> {
    pub fn new(status_code: u32, status_reason: &'a str, content: &'a str) -> Response<'a> {
        Response {
            status_code: status_code,
            status_reason: status_reason,
            content: content
        }
    }
}