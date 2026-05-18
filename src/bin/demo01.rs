use serde_json;

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("{}", serde_json::to_string(&v).unwrap());

    let v = vec![vec![1, 2, 3, 4], vec![2, 3, 4, 5]];
    println!("{}", serde_json::to_string(&v).unwrap());
}
