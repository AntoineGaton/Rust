fn main() {
    // primitive datatypes
    // rust breaks them into two groups: scakar and compound
    
    // scalar datatypes
    // integer
    // i8
    // i16
    // i32
    // i64
    // i128
    let i:i32 = 9999;
    println!("{}", i);

    // unsigned integer
    // this takes only positive numbers
    let u: u32 = 972;
    println!("{}", u);

    // floating point
    let f: f64 = 3.141592653589793;
    println!("{}", f);

    // boolean
    let is_true: bool = true;
    let is_fale: bool = false;
    println!("{}|{}", is_true, is_fale);

    // char
    let letter: char = 'a';
    let exclamation: char = '!';
     println!("{}{}", letter, exclamation);

    // compound datatypes
    //tuples
    let mut tups: (&str,i32,bool)= ("Antoine", 33, true);
    println!("{}", tups.0);
    println!("{}", tups.1);
    tups.2 = false;
    println!("{}", tups.2);

    //array
    let mut arr = [1,2,3,4,5,6,7,8,9,10];
    arr[5] = 100;
    println!("That length of the array is...{}", arr.len());
    println!("{}", arr[5]);
}
