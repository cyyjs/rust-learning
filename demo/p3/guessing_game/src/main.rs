use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("神秘数字为: {secret_number}");
    println!("请输入你的猜测.");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字!");
                continue;
            },
        };
        // let guess: u32 = guess.trim().parse().expect("请输入数字!");
        println!("你猜测: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("你赢了!");
                break;
            }
        }
    }
}
