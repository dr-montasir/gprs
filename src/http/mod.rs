/// ### status_map(status_code)
///
/// The `status_map` function takes an HTTP status code as a u16 (unsigned 16-bit integer) input and returns a string slice (&'static str) representing the corresponding HTTP status message. It maps standard HTTP status codes (e.g., 200, 404) to their associated text descriptions (e.g., "OK", "Not Found"). If the provided code does not match any known status code, it returns "Unknown Status Code".
///
/// ### Sources
/// - https://datatracker.ietf.org/doc/html/rfc7231
/// - https://datatracker.ietf.org/doc/html/rfc4918
/// - https://datatracker.ietf.org/doc/html/rfc6585
/// - https://datatracker.ietf.org/doc/html/rfc7725
/// - https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml
/// - https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
///
/// ### Examples
///
/// ```rust
/// use gprs::http::status_map;
///
/// fn main() {
///     let status_code = 404;
///     let message = status_map(status_code);
///     println!("Status Code: {}, Message: {}", status_code, message);
/// } // Status Code: 404, Message: Not Found
/// ```
///
/// ```rust
/// use gprs::http::status_map;
///
/// fn main() {
///     let codes = [200, 201, 404, 500, 418]; // Example status codes
///     for &code in &codes {
///         println!("Status Code: {}, Message: {}", code, status_map(code));
///     }
/// }
/// // Status Code: 200, Message: OK
/// // Status Code: 201, Message: Created
/// // Status Code: 404, Message: Not Found
/// // Status Code: 500, Message: Internal Server Error
/// // Status Code: 418, Message: I'm a Teapot (RFC3230)
/// ```
/// <small>End Fun Doc</small>
pub fn status_map(status_code: u16) -> &'static str {
    match status_code {
        100 => "Continue",
        101 => "Switching Protocols",
        102 => "Processing",
        103 => "Early Hints",
        104..=199 => "Unassigned",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        207 => "Multi-Status",
        208 => "Already Reported",
        209 => "Content Range Not Satisfiable",
        210 => "Content Version Identifier",
        211..=225 => "Unassigned",
        226 => "IM Used",
        227..=299 => "Unassigned",
        300 => "Multiple Choices",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        304 => "Not Modified",
        305 => "Use Proxy",
        306 => "Switch Proxy",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        309..=399 => "Unassigned",
        400 => "Bad Request",
        401 => "Unauthorized",
        402 => "Payment Required",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        406 => "Not Acceptable",
        407 => "Proxy Authentication Required",
        408 => "Request Timeout",
        409 => "Conflict",
        410 => "Gone",
        411 => "Length Required",
        412 => "Precondition Failed",
        413 => "Payload Too Large",
        414 => "URI Too Long",
        415 => "Unsupported Media Type",
        416 => "Range Not Satisfiable",
        417 => "Expectation Failed",
        418 => "I'm a Teapot (RFC3230)",
        419..=421 => "Unassigned",
        422 => "Unprocessable Entity (RFC4918)",
        423 => "Locked (RFC4918)",
        424 => "Failed Dependency (RFC4918)",
        425 => "Unordered Collection (RFC4918)",
        426 => "Upgrade Required (RFC2817)",
        427 => "Unassigned",
        428 => "Precondition Required (RFC6585)",
        429 => "Too Many Requests (RFC6585)",
        430 => "Unassigned",
        431 => "Request Header Fields Too Large (RFC6585)",
        432..=450 => "Unassigned",
        451 => "Unavailable For Legal Reasons (RFC7725)",
        452..=499 => "Unassigned",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        505 => "HTTP Version Not Supported",
        506 => "Variant Also Negotiates (RFC2295)",
        507 => "Insufficient Storage (RFC4918)",
        508 => "Loop Detected (RFC5842)",
        509 => "Bandwidth Limit Exceeded (Apache specific)",
        510 => "Not Extended (RFC2774)",
        511 => "Network Authentication Required (RFC6585)",
        512..=599 => "Unassigned",
        _ => "Unknown Status Code",
    }
}
