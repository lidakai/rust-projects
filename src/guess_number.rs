use rand::Rng;
use std::{cmp::Ordering, io};

pub fn guess_number() {
    println!("猜数字游戏!");

    let rand_number = rand::thread_rng().gen_range(1..10);

    // println!("神秘数字是{}",secret_number);

    println!("猜神秘数字是多少？");

    // 创建个空的字符串实例。
    let check_result = cycle_check(rand_number);

    println!("------- check result {} --------", check_result);

    println!("输入任意值退出！");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");
}

fn cycle_check(rand_number: usize) -> bool {
    loop {
        let guess = create_read_line();
        let is_pass = is_pass(rand_number, guess);
        if is_pass {
            break true;
        }
    }
}

fn is_pass(rand_number: usize, guess: usize) -> bool {
    match guess.cmp(&rand_number) {
        Ordering::Less => {
            println!("你猜测的数是：{} ，太小喽，要继续努力哦！", guess);
            return false;
        }
        Ordering::Greater => {
            println!("你猜测的数是：{} ，太大喽，要继续努力哦！", guess);
            return false;
        }
        Ordering::Equal => {
            println!("好厉害猜中了！~");
            return true;
        }
    }
}

fn create_read_line() -> usize {
    loop {
        // 创建个空的字符串实例。
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");
        if guess.trim().len() > 0 {
            let guess: usize = guess.trim().parse().expect("Not a number!");
            break guess;
        }
    }
}
