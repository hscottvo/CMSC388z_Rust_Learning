fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // v.push(5);
    // v.push(6);
    let mut v: Vec<i32> = vec![5, 6];

    for val in &v {
        println!("value: {}", val);
    }
}
