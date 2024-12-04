fn main() {    let mut vec = Vec::new();    vec.push(1);    vec.push(2);    vec.push(3);    let x = vec[0]; //Copy the value instead of referencing it    vec.push(4);    println!("Value of x: {}", x);}

//Alternative solution using cloning:
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    let x = &vec[0];
    let vec2 = vec.clone(); //Clone the vector to avoid mutation issues
    vec2.push(4);
    println!("Value of x: {}", x);
}