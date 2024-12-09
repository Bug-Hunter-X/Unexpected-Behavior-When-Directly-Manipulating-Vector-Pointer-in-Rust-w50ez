fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 4; // This is a bug.  It modifies the first element of v.
    }
    println!("{:?}", v); //Prints [4,2,3]
}