pub use crate::do_html;

/// ### do_forloop(vector, befor_items, befor_item, after_item, after_items)
///
/// Html Template Function
///
/// The `do_forloop` function takes a vector of items and formats them according to specified prefixes
/// and suffixes for the entire collection, as well as for each individual item. The output
/// is a concatenated string representation of the items, encapsulated within the provided
/// separator strings.
///
/// ### Parameters
/// - `vector`: A slice of items to format, where each item must implement the `Display` trait.
/// - `befor_items`: A string that will be prefixed before the whole collection of items.
/// - `befor_item`: A string that will be prefixed before each individual item.
/// - `after_item`: A string that will be suffixed after each individual item.
/// - `after_items`: A string that will be suffixed after the whole collection of items.
///
/// ### Examples
/// ```rust
/// use gprs::template::html::do_forloop;
///
/// let items = vec!["Apples", "Bananas", "Cherries"];
/// let result = do_forloop(&items, "<ul>", "<li>", "</li>", "</ul>");
/// assert_eq!(result, "<ul><li>Apples</li><li>Bananas</li><li>Cherries</li></ul>");
///
/// let float_vector = vec![1.0, 2.0, 3.0];
/// let forloop_float = do_forloop(&float_vector, "", "", "", "");
/// assert_eq!(forloop_float, "123");
/// ```
/// <small>End Fun Doc</small>
pub fn do_forloop<T: std::fmt::Display>(
    vector: &[T],
    befor_items: &str,
    befor_item: &str,
    after_item: &str,
    after_items: &str,
) -> String {
    let items = vector
        .iter()
        .map(|item| format!("{befor_item}{item}{after_item}"))
        .collect::<Vec<_>>()
        .join("");
    format!("{}{}{}", befor_items, items, after_items)
}
