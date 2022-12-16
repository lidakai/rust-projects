use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("猜数字游戏!");

    let rand_number = rand::thread_rng().gen_range(1..10);

    // println!("神秘数字是{}",secret_number);

    println!("猜神秘数字是多少？");

    // 创建个空的字符串实例。
    let check_result = cycle_check(rand_number);

    println!("------- check result {} --------",check_result);
}

fn cycle_check (rand_number:usize) -> bool {
    let guess = create_read_line();
    let is_pass = is_pass(rand_number,guess);
    if is_pass == true {
        return  true;
    }else{
        return  cycle_check(rand_number);
    }
}

fn is_pass(rand_number:usize,guess:usize) -> bool {
    match guess.cmp(&rand_number) {
        Ordering::Less => {
            println!("你猜测的数是：{} ，太小喽，要继续努力哦！", guess);
            return  false;
        },
        Ordering::Greater => {
            println!("你猜测的数是：{} ，太大喽，要继续努力哦！", guess);
            return  false;
        },
        Ordering::Equal => {
            println!("好厉害猜中了！~");
            return  true;
        }
    }
}

fn create_read_line () ->usize{
     // 创建个空的字符串实例。
     let mut guess = String::new();

     io::stdin().read_line(&mut guess).expect("无法读取行");
 
     let  guess:usize = guess.trim().parse().expect("Not a number!");
    return guess;
}