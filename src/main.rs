use gprs::asyncore::task;
// use gprs::cans::do_html;
use gprs::cans::html::{do_forloop, do_html, do_text};
use gprs::http::status_map;
use std::time::Duration;

async fn print_numbers(from: i32, to: i32) {
    for i in from..to {
        println!("{}", i32::abs(i));

        task::sleep(Duration::from_millis(1000)).await
    }
}

pub const HEAD: &str = r#"<head>
<meta charset="UTF-8">
    <title>{{page_title}} Page</title>
</head>"#;

pub const HOME_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
  {{HEAD}}
  <body>
     Home Page
  </body>
</html>"#;

pub fn do_home_page() -> String {
    do_html!(HOME_TEMPLATE, HEAD = HEAD, page_title = do_text("Home"))
}

pub const ABOUT_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
  {{HEAD}}
  <body>
     About Page
  </body>
</html>"#;

pub fn do_about_page() -> String {
    do_html!(ABOUT_TEMPLATE, HEAD = HEAD, page_title = do_text("About"))
}

#[gprs::main]
async fn main() {
    let task_1 = task::spawn(print_numbers(-3, 0));
    task::block_on(task_1);

    println!("{}", do_text(r#"Hello, World!"#));

    let status_code = 200;
    let message = status_map(status_code);

    println!("Status Code: {}, Message: {}", status_code, message);

    let html_template = r#"
    <!DOCTYPE html>
    <html>
        <head>
            <meta charset="UTF-8">
            <title>GPRS | {{page_title}}</title>
        </head>
        <ol>
        </ol>
        <h1>👋 {{hello}}, {{world}}</h1>
        <p>
            {{component_if}}
        </p>
        <div>{{forloop_float}}<div>
        <div>{{forloop_float_ol}}<div>
    </html>
    "#;

    let world = "World!";

    let component_if: &str;
    let x = 3;

    if x == 1 {
        component_if = "<a href='#'><i>x = 1</i></a>";
    } else if x == 2 {
        component_if = "<a href='#'><i>x = {{x}}</i></a>";
    } else {
        component_if = "<a href=\"#\"><i>x ≠ 1 & x  ≠ 2. The 'x' value is ( {{x}} )</i></a>";
    };

    let float_vector = vec![1.0, 2.0, 3.0];
    let forloop_float = do_forloop(&float_vector, "", "", "", "");
    let forloop_float_ol = do_forloop(
        &float_vector,
        "<ol style='list-style: square;'>",
        "<li>",
        "</li>",
        "</ol>",
    );

    let html_to_string = do_html!(
        html_template,
        page_title = do_text("Home Page"),
        hello = "Hello",
        world = world,
        component_if = component_if,
        x = x,
        forloop_float = forloop_float,
        forloop_float_ol = forloop_float_ol
    );

    println!("{}", do_home_page());

    println!("{}", html_to_string);

    println!("{}", do_about_page());

    let task_2 = task::spawn(print_numbers(1, 4));
    task::block_on(task_2);
}
