fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 100; // Safe and idiomatic way to modify vector elements
    println!("{:?}", v);
}