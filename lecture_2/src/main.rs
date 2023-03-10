fn main() {
    // basic initialization
    // let mut v: Vec<i32> = Vec::new();
    // v.push(5);
    // v.push(6);

    // easier vector constructor
    // let mut v: Vec<i32> = vec![5, 6];

    // 8-size with value 4
    let v: Vec<i32> = vec![4; 8];

    for val in &v {
        println!("value: {}", val);
    }
}
