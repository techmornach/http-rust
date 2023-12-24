# Rust HTTP Library

This Rust library provides basic functionality for handling HTTP requests and generating HTTP responses. It consists of two modules: `httprequest` for parsing incoming HTTP requests and `httpresponse` for creating HTTP responses.

## `httprequest` Module

### Structs
#### `Resource`

- Represents the resource path in an HTTP request.

#### `HttpRequest`

- Represents an incoming HTTP request.
- Fields:
  - `method`: HTTP method (`Get`, `Post`, or `Uninitialized`).
  - `version`: HTTP version (`V1_1`, `V2_0`, or `Uninitialized`).
  - `resource`: Resource path (`Path`).
  - `headers`: HashMap containing request headers.
  - `msg_body`: String containing the message body.

### Methods
#### `impl From<String> for HttpRequest`

- Converts a raw HTTP request string into an `HttpRequest` struct.

#### `fn process_req_line(s: &str) -> (Method, Resource, Version)`

- Processes the request line to extract method, resource, and version.

#### `fn process_req_header(s: &str) -> (String, String)`

- Processes a request header line to extract key and value.

### Enums
#### `Method`

- Represents HTTP methods (`Get`, `Post`, or `Uninitialized`).

#### `Version`

- Represents HTTP versions (`V1_1`, `V2_0`, or `Uninitialized`).

### Tests
- Unit tests are provided in the `tests` module to ensure proper functionality.

## `httpresponse` Module

### Struct
#### `HttpResponse<'a>`

- Represents an HTTP response.
- Fields:
  - `version`: HTTP version.
  - `status_code`: HTTP status code.
  - `status_text`: HTTP status text.
  - `headers`: Optional HashMap containing response headers.
  - `body`: Optional response body.

### Methods
#### `fn new(status_code: &'a str, headers: Option<HashMap<&'a str, &'a str>>, body: Option<String>) -> HttpResponse<'a>`

- Creates a new `HttpResponse` instance with the specified status code, headers, and body.

#### `fn send_response(&self, write_stream: &mut impl Write) -> Result<()>`

- Sends the HTTP response to the provided write stream.

### From Trait Implementation
#### `impl<'a> From<HttpResponse<'a>> for String`

- Converts an `HttpResponse` into a formatted HTTP response string.

### Tests
- Unit tests are provided in the `tests` module to ensure proper functionality.

## Usage Example

```rust
use http::httprequest::HttpRequest;
use http::httpresponse::HttpResponse;
use std::collections::HashMap;

fn main() {
    // Parse incoming HTTP request
    let request_str = "GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n";
    let request: HttpRequest = request_str.into();

    // Create an HTTP response
    let headers = Some({
        let mut h = HashMap::new();
        h.insert("Content-Type", "text/html");
        h
    });
    let body = Some("Hello, world!".to_string());
    let response = HttpResponse::new("200", headers, body);

    // Send the response
    let mut write_stream = Vec::new();
    response.send_response(&mut write_stream).unwrap();
    let response_str = String::from_utf8(write_stream).unwrap();

    println!("Received Request: {:?}", request);
    println!("Generated Response: {}", response_str);
}
```

Feel free to customize and extend the library based on your specific use case and requirements. If you have any questions or encounter issues, please refer to the provided unit tests or reach out for support.