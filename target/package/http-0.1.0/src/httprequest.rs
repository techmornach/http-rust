use std::collections::HashMap;
#[derive(PartialEq, Debug)]
pub enum Resource{
    Path(String)
}

#[derive(PartialEq, Debug)]
pub struct HttpRequest{
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}
impl From<String> for HttpRequest{
    fn from(req:String) -> Self
    {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines()
        {
            if line.contains("HTTP")
            {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            }
            else if line.contains(":")
            {
                let (key, value) = process_req_header(line);
                parsed_headers.insert(key, value);
            }
            else if line.len() == 0
            {

            }
            else{
                parsed_msg_body = line;
            }

        }
        HttpRequest{
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) ->(Method, Resource, Version)
{
    let mut word = s.split_whitespace();
    let method = word.next().unwrap();
    let resource = word.next().unwrap();
    let version = word.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into()
    )
}

fn process_req_header(s: &str) -> (String, String)
{
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = header_items.next(){
        key = k.to_string();  
    }

    if let Some(v) = header_items.next()
    {
            value = v.to_string();
    }
    (key, value)
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Method{
    Get,
    Post,
    Uninitialized
}
impl From<&str> for Method{
    fn from(s: &str) -> Method{
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}


#[derive(PartialEq, Debug)]
pub enum Version{
    V1_1,
    V2_0,
    Uninitialized,
}
impl From<&str> for Version{
    fn from(s: &str) -> Version{
        match s{
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}


#[cfg(test)]

mod tests{
    use super::*;
    #[test]
    fn test_method_into()
    {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }
    #[test]
    fn test_version_into()
    {
        let m:Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }
    #[test]
    fn test_incoming_http_request()
    {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
    let mut headers_expected = HashMap::new(); 
    headers_expected.insert("Host".into(), " localhost".into()); 
    headers_expected.insert("Accept".into(), " */*".into()); 
    headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());  
    let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}