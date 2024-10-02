use gprs::asyncore::task;
use gprs::template::t;
use std::time::Duration;

async fn print_numbers(from: i32, to: i32) {
    for i in from..to {
        println!("{}", i32::abs(i));

        task::sleep(Duration::from_millis(1000)).await
    }
}

#[gprs::main]
async fn main() {
    let task_1 = task::spawn(print_numbers(-3, 0));
    task::block_on(task_1);

    println!("{}", t(r#"Hello, World!"#));

    let task_2 = task::spawn(print_numbers(1, 4));
    task::block_on(task_2);
}
