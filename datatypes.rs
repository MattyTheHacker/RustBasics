fn primatives(){
    /* 
    integers:
    types that begin with an i are signed integers and can be positive or negative
    types that begin with an u are unsigned integers and can only be positive 
    */
    let a: i8 = -1;
    let b: u8 = 1;
    let c: i16 = -1;
    let d: u16 = 1;
    let e: i32 = -1;
    let f: u32 = 1;
    let g: i64 = -1;
    let h: u64 = 1;
    let i: i128 = -1;
    let j: u128 = 1;
    
    /*
    floats:
    floats can not be unsigned
    the default value is f64
    */
    let k: f32 = 1.0;
    let l: f64 = -1.0;

    /*
    boolean:
    booleans can be either true or false, 1 or 0
    */
    let m: bool = true;
    let n: bool = false;
    let o: bool = 1;
    let p: bool = 0;

    /*
    char:
    a single character, must be surrounded with single quotes
    */
    let q: char = 'a';

}

fn compound(){
    /*
    tuples:
    a tuple is a collection of values separated by commas
    */
    let tup: (i32, bool, char) = (1, true, 'a');
    println!("{}", tup.1);
    println!("{},{},{}", tup.0, tup.1, tup.2);

    let mut mutable_tuple: (i32, bool, char) = (2, false, 'b');
    mutable_tuple.0 = 3;
    mutable_tuple.1 = true;
    mutable_tuple.2 = 'c';
    println!("{},{},{}", mutable_tuple.0, mutable_tuple.1, mutable_tuple.2);


    /*
    arrays:
    arrays are fixed size, arrays are always homogenous
    */
    let arr: [i32; 3] = [1, 2, 3];
    println!("{}", arr[1]);
    println!("{},{},{}", arr[0], arr[1], arr[2]);

    let mut mutable_arr: [i32; 3] = [4, 5, 6];
    mutable_arr[1] = 8;
    println!("{}", mutable_arr[1]);
    println!("{},{},{}", mutable_arr[0], mutable_arr[1], mutable_arr[2]);
}