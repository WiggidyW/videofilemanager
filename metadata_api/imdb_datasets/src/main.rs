fn main() {
    let v = vec!["one", "two", "three", "four", "five"];
    for val in v.iter().enumerate().rev() {
        println!("{:?}", val);
    }
}