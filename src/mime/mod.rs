use std::collections::HashMap;

/// ### set_mime_types()
///
/// Initializes a HashMap with common file extensions and their corresponding MIME types. It returns this map.
pub fn set_mime_types() -> HashMap<String, String> {
    let mime_types: HashMap<String, String> = [
        ("html", "text/html"),
        ("css", "text/css"),
        ("js", "application/javascript"),
        ("png", "image/png"),
        ("jpg", "image/jpeg"),
        ("svg", "image/svg+xml"),
        ("gif", "image/gif"),
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
