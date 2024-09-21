use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("숫자를 맞춰보세요!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("값을 입력하세요.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("값을 읽어오지 못했습니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("값이 올바르지 않습니다.");
                continue;
            }
        };

        println!("입력한 값은 : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("너무 작아요!"),
            Ordering::Greater => println!("너무 커요!"),
            Ordering::Equal => {
                println!("정답입니다!");
                break;
            }
        }
    }
}
