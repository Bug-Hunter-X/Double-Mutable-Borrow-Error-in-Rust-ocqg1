fn main() {
    let mut x = 5;
    { //Creating a scope for y
        let y = &mut x;
        *y += 1; 
    }
    { //Creating a scope for z
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}
