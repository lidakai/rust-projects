mod constant;
// mod guess_number;
// mod while_test;

fn main() {
    // while_test::while_test();
    // while_test::for_test();

    // guess_number::guess_number();
    let mut c = constant::constant::Constant::new();
    let key = String::from("likai");
    let value = String::from("一定会成功");
    c.set(key, value);

    let getKey = key;
    let hash_result = c.get(getKey);
    println!("响应结果，{:?}", hash_result);
}
