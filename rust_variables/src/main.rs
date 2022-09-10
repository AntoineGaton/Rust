fn main() {
    // definign an implicit variable
    let a = 3;
    println!("a is: {}", a);

    // definign an explicit variable
    let b:u32 = 4;
    println!("b is: {}", b);

    // NOTE: cannot assign twice to immutable variable
    // let x = 4;
    // x =5;

    // declaring a variable that is mutable
    let mut c = 4;
    println!("c is: {}", c);

    c = c*2;
    println!("c is: {}", c);

    // overriding a variable
    let d = 10;
    println!("d is: {}", d);

    let d = 100;
    println!("d is: {}", d);

    // name shadowing
    let e = 8;
    println!("e is: {}", e);

    {
        let e = e+10;
        println!("e is: {}", e);
    }

    let e = e+2;
    println!("e is: {}", e);

    // constant variable
    const THIS_CANNOT_CHANGE:u32 = 0;
    // this cannot be reassignled like a regular variable
    // const THIS_CANNOT_CHANGE:u32 = 100; // this will give an error
    println!("{}",THIS_CANNOT_CHANGE)
}
