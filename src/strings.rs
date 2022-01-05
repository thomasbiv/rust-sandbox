//Primitive str = Immutable fixed-length string somewhere in memory
//String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let hello_prim = "Hello"; //primitive
    let mut hello = String::from("Hello "); //growable

    //Get length
    println!("Length: {}", hello.len());

    hello.push('W'); //Push a char
    hello.push_str("orld!"); //Push a string
    println!("{}", hello);

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Check if empty
    println!("Is empty: {}", hello.is_empty());

    //Containts
    println!("Contains 'World': {}", hello.contains("World"));

    //Replace
    println!("Replace: {}", hello.replace("World", "There"));

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    //Assertion testing
    assert_eq!(2, s.len()); //Testing if these to vals are equal (does nothing if success, throws error if fails)
    //assert_eq!(3, s.len()); //THIS WILL FAIL

    println!("{} {}", hello_prim, hello);

}