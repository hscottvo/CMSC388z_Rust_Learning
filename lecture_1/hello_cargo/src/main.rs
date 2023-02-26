fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_2: [i32; 5] = [3; 5];
    println!("{:?}", &arr[0] + &arr_2[3]);
}
