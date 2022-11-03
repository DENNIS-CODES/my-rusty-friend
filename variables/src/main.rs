fn main() {
    // println! is a macro that prints text to the console
    println!("Hello World!");
    // immutable variable
    let x = 5;
    // mutable variable
    let mut y = 5;
    y = 6;
    println!("The value of x is: {x}");
    println!("The value of x is: {y}");
    // constant
    const MAX_POINTS: u32 = 100_000;
    // shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {z}");
    // shadowing with different types
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
    // data types
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");
    // floating point types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
                      // numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("The value of sum is: {sum}");
    println!("diffrence: {difference}");
    println!("product: {product}");
    println!("quotient: {quotient}");
    println!("remainder: {remainder}");
    // boolean type
    let t = true;
    let f: bool = false; // with explicit type annotation
                         // character type
    let c = 'z';
    let z = 'ℤ';
    println!("The value of c is: {c}");
    println!("z: {z}");
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");
    // compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");
    // array type/tuple type
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");
    // array type
    let a = [3; 5];
    println!("The value of a is: {a}");
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");
    // function
    another_function();
    // function with parameters
    another_function_with_parameters(5);
    // function with return value
    let x = five();
    println!("The value of x is: {x}");
    // function with return value and parameters
    let x = plus_one(5);
    println!("The value of x is: {x}");
    // if expressions
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // if expressions with else if
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // if expressions with if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    // loop
    loop {
        println!("again!");
    }
    // loop with break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");
    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    // for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    // for with range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    // ownership
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    println!("The length of '{}' is {}.", s5, len);
    // references and borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // mutable references
    let mut s = String::from("hello");
    change(&mut s);
    println!("The value of s is: {s}");
    // dangling references
    let reference_to_nothing = dangle();
    println!("The value of reference_to_nothing is: {reference_to_nothing}");
    // slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("The value of hello is: {hello}");
    println!("The value of world is: {world}");
}
