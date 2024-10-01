use gprs::template::t;

#[gprs::main]
async fn main() {
    println!("{}", t(r#"Hello, World!"#));
}
