pub fn while_test() {
    let end_num = 1;
    let mut start_num = 10;

    while start_num >= end_num {
        println!("{}", start_num);
        start_num -= 1;
    }
    println!("while end");
}

pub fn for_test() {
    for ele in (1..11).rev() {
        println!("{}", ele);
    }
    println!("for end")
}
