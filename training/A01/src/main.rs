fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).expect("Failed to read from stdin");
    let mut stdin = stdin.split_whitespace();
    let n: usize = stdin.next().unwrap().parse().expect("Failed to parse n");

    let out = n * n;
    println!("{}", out);
}
