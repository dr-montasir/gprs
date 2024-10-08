use std::collections::HashMap;

/// ### set_mime_types()
///
/// Initializes a HashMap with common file extensions and their corresponding MIME types. It returns this map.
pub fn set_mime_types() -> HashMap<String, String> {
    let mime_types: HashMap<String, String> = [
        ("html", "text/html"),
        ("ejs", "text/html"),
        ("css", "text/css"),
        ("scss", "text/scss"),
        ("less", "text/css"),
        ("csv", "text/csv"),
        ("rs", "text/plain"),
        ("plain", "text/plain"),
        ("txt", "text/plain"),
        ("markdown", "text/markdown"),
        ("md", "text/markdown"),
        ("xml", "text/xml"),
        ("yaml", "text/yaml"),
        ("wat", "text/wat"),
        ("py", "text/x-python"),
        ("log", "text/plain"),
        ("ics", "text/calendar"),
        ("vcard", "text/vcard"),
        ("php", "text/x-php"),
        ("conf", "text/plain"),
        ("init", "text/plain"),
        ("png", "image/png"),
        ("jpg", "image/jpeg"),
        ("jpeg", "image/jpeg"),
        ("gif", "image/gif"),
        ("svg", "image/svg+xml"),
        ("svgz", "image/svg+xml"),
        ("bmp", "image/bmp"),
        ("tiff", "image/tiff"),
        ("ico", "image/x-icon"),
        ("webp", "image/webp"),
        ("woff", "font/woff"),
        ("woff2", "font/woff2"),
        ("ttf", "font/ttf"),
        ("otf", "font/otf"),
        ("eot", "application/vnd.ms-fontobject"),
        ("js", "application/javascript"),
        ("ts", "application/typescript"),
        ("xml", "application/xml"),
        ("json", "application/json"),
        ("geojson", "application/geo+json"),
        ("map", "application/json-patch+json"),
        ("jsonld", "application/ld+json"),
        ("binary", "application/octet-stream"),
        ("tar", "application/x-tar"),
        ("so", "application/octet-stream"),
        ("so", "application/x-sharedlib"),
        ("deb", "application/vnd.debian.binary-package"),
        ("bash", "application/x-shellscript"),
        ("php", "application/x-httpd-php"),
        ("db", "application/x-database"),
        ("sql", "application/sql"),
        ("pdf", "application/pdf"),
        ("zip", "application/zip"),
        ("gz", "application/gzip"),
        ("epub", "application/epub+zip"),
        ("toml", "application/toml"),
        ("xhtml", "application/xhtml+xml"),
        ("rss", "application/rss+xml"),
        ("atom", "application/atom+xml"),
        ("graphql", "application/graphql"),
        ("form", "application/x-www-form-urlencoded"),
        ("srt", "application/x-subrip"),
        ("rtf", "application/rtf"),
        ("kml", "application/vnd.google-earth.kml+xml"),
        ("kmz", "application/vnd.google-earth.kmz"),
        ("sh", "application/x-sh"),
        ("bin", "application/octet-stream"),
        ("exe", "application/octet-stream"),
        ("dll", "application/octet-stream"),
        ("class", "application/java-vm"),
        ("jar", "application/java-archive"),
        ("wasm", "application/wasm"),
        ("wat", "application/wasm"),
        ("conf", "application/x-configuration"),
        ("init", "application/x-initialization"),
        ("pem", "application/x-x509-ca-cert"),
        ("p12", "application/x-pkcs12"),
        ("p7b", "application/x-pkcs7-certificates"),
        ("p7c", "application/pkcs7-mime"),
        ("pot", "application/vnd.ms-powerpoint"),
        ("doc", "application/msword"),
        (
            "docx",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        ),
        ("xls", "application/vnd.ms-excel"),
        (
            "xlsx",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ),
        ("ppt", "application/vnd.ms-powerpoint"),
        (
            "pptx",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        ),
        (
            "dotx",
            "application/vnd.openxmlformats-officedocument.wordtemplate",
        ),
        ("xla", "application/vnd.ms-excel"),
        ("xlt", "application/vnd.ms-excel"),
        (
            "xltx",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.template",
        ),
        ("bat", "application/bat"),
        ("pub", "application/x-mspublisher"),
        ("xps", "application/vnd.ms-xpsdocument"),
        ("msg", "application/vnd.ms-outlook"),
        ("dot", "application/msword"),
        ("wps", "application/vnd.ms-works"),
        ("odt", "application/vnd.oasis.opendocument.text"),
        ("ods", "application/vnd.oasis.opendocument.spreadsheet"),
        ("apk", "application/vnd.android.package-archive"),
        ("iot", "application/x-iot-file"),
        ("iot", "application/x-iot-data"),
        ("application", "application/octet-stream"),
        ("wav", "audio/wav"),
        ("mp3", "audio/mpeg"),
        ("aac", "audio/aac"),
        ("ogg", "audio/ogg"),
        ("m4a", "audio/mp4a-latm"),
        ("flac", "audio/flac"),
        ("midi", "audio/midi"),
        ("mid", "audio/midi"),
        ("mp4", "video/mp4"),
        ("webm", "video/webm"),
        ("mov", "video/quicktime"),
        ("avi", "video/x-msvideo"),
        ("flv", "video/x-flv"),
        ("mkv", "video/x-matroska"),
        ("mpg", "video/mpeg"),
        ("mpeg", "video/mpeg"),
        ("mpa", "video/x-mpeg"),
    ]
    .iter()
    .map(|&(ext, mime)| (ext.to_string(), mime.to_string()))
    .collect();

    mime_types
}

/// ### insert_mime_type()
///
/// Allows inserting a single MIME type into an existing HashMap for the given file extension.
pub fn insert_mime_type(
    mime_types: &mut HashMap<String, String>,
    extension: &str,
    mime_type: &str,
) {
    mime_types.insert(extension.to_string(), mime_type.to_string());
}

/// remove_mime_type()
///
/// Removes a MIME type for a specified file extension from the HashMap.
pub fn remove_mime_type(mime_types: &mut HashMap<String, String>, extension: &str) {
    mime_types.remove(extension);
}

/// ### insert_mime_types()
///
/// Takes another HashMap of MIME types and inserts all its entries into the existing HashMap.
pub fn insert_mime_types(
    mime_types: &mut HashMap<String, String>,
    mime_types_map: HashMap<String, String>,
) {
    for (extension, mime_type) in mime_types_map {
        mime_types.insert(extension, mime_type);
    }
}

/// ### remove_mime_types()
///
/// Removes all MIME types from the existing HashMap that match the extensions in the provided map.
pub fn remove_mime_types(
    mime_types: &mut HashMap<String, String>,
    mime_types_map: HashMap<String, String>,
) {
    for extension in mime_types_map.keys() {
        mime_types.remove(extension);
    }
}

/// ### display_mime_types()
///
/// Prints all the current MIME types stored in a HashMap to the console.
pub fn display_mime_types(mime_types: &HashMap<String, String>) {
    for (extension, mime_type) in mime_types {
        println!("Extension: {} - MIME Type: {}", extension, mime_type);
    }
}

/// ### default_mime_types()
///
/// Displays the default set of MIME types by calling the set_mime_types() function and printing its contents.
pub fn default_mime_types() {
    println!("\nDefault MIME types:");
    let default_mime_types = set_mime_types();
    for (extension, mime_type) in &default_mime_types {
        println!("Extension: {} - MIME Type: {}", extension, mime_type);
    }
}
