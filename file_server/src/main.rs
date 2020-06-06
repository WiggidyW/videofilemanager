fn main() {
    let res = std::fs::copy("test1.txt", "test2.txt");
    println!("{:?}", res);
}