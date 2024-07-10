use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guess(n: u32, secret_number: u32) -> String {
    match n.cmp(&secret_number) {
        Ordering::Less => "太小了!".to_string(),
        Ordering::Greater => "太大了!".to_string(),
        Ordering::Equal => "恭喜獲勝!".to_string(),
  }
}

fn main() {
    println!("1~10猜數字！");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("請輸入數字");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("解析失敗");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的數字是: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("恭喜獲勝!");
                break;
            }
        }
    }
}

