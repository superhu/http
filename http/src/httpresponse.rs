use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl <'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self{
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String{
    fn from(resp: HttpResponse<'a>) -> Self {
        let res = resp.clone();
        format!(
            "{} {} {}\r\nContent-length: {}\r\n{}\r\n\r\n{}",
            &res.version(),
            &res.status_code(),
            &res.status_text(),
            &res.body.clone().unwrap_or("".to_string()).len(),
            &res.headers(),
            &res.body(),
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new (status_code: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.clone();
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut map = HashMap::new();
                map.insert("Content-Type", "text/html");
                Some(map)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };
        response.body = body;

        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<(), &'static str> {
        let res = self.clone();
        let response_string = String::from(res);
        // write_stream.write(response_string.as_bytes()).expect("write fail");
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }

    pub fn version(&self) -> &str {
        self.version
    }

    pub fn status_code(&self) -> &str {
        self.status_code
    }
    pub fn status_text(&self) -> &str {
        self.status_text
    }

    pub fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut header_string: String = "".to_string();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k , v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) =>b.as_str(),
            None => "",
        }
    }

}