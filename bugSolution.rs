fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; // Safer way to modify the first element
    println!("{:?}", v); //Prints [4,2,3]
}