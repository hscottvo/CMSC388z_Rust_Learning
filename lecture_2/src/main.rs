fn main() {
    let mut v: Vec<i32> = vec![3; 6];

    v.push(5);
    for i in &mut v {
        println!("{:}", i);
    }

    println!("");

    for i in &mut v[3..5] {
        println!("{:}", i);
    }
}
