fn main() {
    let s_literals = "hello";
    let s1 = String::from(s_literals);
    let s = &s1;
    let s_slice = &s1[..];
}
