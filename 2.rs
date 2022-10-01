fn main() {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).expect(" error ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect(" error ");

    let vec: Vec<i32> = input
        .trim().split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    print!("{} ", vec[vec.len() - 1]);
    for i in 0..vec.len() - 1 {
        print!("{} ", vec[i]);
    }
}
