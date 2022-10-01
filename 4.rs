fn main() {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("error");
    let mut arr = String::new();
    std::io::stdin().read_line(&mut arr).expect("error");
    let arr: Vec<i32> = arr
        .trim().split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    let mut counter = 0i32;
    for i in 1..arr.len()-1 {
        if arr[i] > arr[i - 1] && arr[i] > arr[i + 1] { counter += 1 }
    }
    println!("{}", counter)
}
