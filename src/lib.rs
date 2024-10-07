/// ### Asyncore Module
pub mod asyncore;

/// ### Http Module
pub mod http;

/// ### Mime Module
///
/// A set of functions for managing MIME types using a HashMap.
///
/// Examples
///
/// ```rust
/// use gprs::mime::{
///     default_mime_types, display_mime_types,
///     insert_mime_type, remove_mime_type,
///     set_mime_types,
/// };
///
/// fn main() {
///    // Create a new HashMap and initialize it with default MIME types
///    let mut mime_types = set_mime_types();
///    
///    println!("Initial MIME types:");
///    display_mime_types(&mime_types);
///
///    // Insert a new MIME type
///    insert_mime_type(&mut mime_types, "txt", "text/plain");
///    println!("\nAfter inserting .txt MIME type:");
///    display_mime_types(&mime_types);
///
///    // Remove an existing MIME type
///    remove_mime_type(&mut mime_types, "jpg");
///    println!("\nAfter removing .jpg MIME type:");
///    display_mime_types(&mime_types);
///
///    // Display default MIME types
///    default_mime_types();
/// }
/// ```
pub mod mime;

/// ### Parser Module
pub mod parser;

/// ### Runtime Module
pub mod runtime;

/// ### Template Module
pub mod template;

pub use runtime::*;
