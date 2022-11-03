fn main() {
    // println! is a macro that prints text to the console
    println!("Hello World!");
    // immutable variable
    let x = 5;
    // mutable variable
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of x is: {x}");
    println!("The value of x is: {y}");
    // constant
    const _MAX_POINTS: u32 = 100_000;
    fn shadowing_ex() {
        // shadowing
        let z = 5;
        let z = z + 1;
        let z = z * 2;
        println!("The value of z is: {z}");
        // shadowing with different types
        let spaces = "   ";
        let spaces = spaces.len();
        println!("The value of spaces is: {spaces}");
    }
    shadowing_ex();

    fn ex_data_types() {
        // data types
        let guess: u32 = "42".parse().expect("Not a number!");
        println!("The value of guess is: {guess}");
        // floating point types
        let _x = 2.0; // f64
        let _y: f32 = 3.0; // f32
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
        let _t = true;
        let _f: bool = false; // with explicit type annotation
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
        println!("The value of x is: {x}");
        println!("The value of y is: {y}");
        println!("The value of z is: {z}");
        let five_hundred = tup.0;
        let six_point_four = tup.1;
        let one = tup.2;
        println!("The value of five_hundred is: {five_hundred}");
        println!("The value of six_point_four is: {six_point_four}");
        println!("The value of one is: {one}");
    }
    ex_data_types();
    // array type/tuple type
    fn ex_array() {
        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];
        println!("The value of first is: {first}");
        println!("The value of second is: {second}");
        // array type
        let a = [3; 5];
        let first = a[0];
        let second = a[1];
        println!("The value of first is: {first}");
        println!("The value of second is: {second}");
    }
    ex_array();
    // function
    fn another_function() {
        println!("Another function.");
    }
    another_function();
    // function with parameters
    fn another_function_with_parameters(some_number: i32) -> i32 {
        let x = some_number + 1;
        let sum = x + 1;
        return sum;
    }

    // function with return value
    let x = another_function_with_parameters(5);
    println!("The value of x is: {x}");
    // function with return value and parameters
    let x = plus_one(5);
    println!("The value of x is: {x}");

    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    fn ex_control_flow() {
        // if expressions
        let number = 3;
        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }
    ex_control_flow();

    fn ex_control_flow_else_if() {
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
    }
    ex_control_flow_else_if();

    fn ex_if_in_let_statement() {
        // if expressions with if in a let statement
        let condition = true;
        let number = if condition { 5 } else { 6 };
        println!("The value of number is: {number}");
    }
    ex_if_in_let_statement();

    fn ex_loop() {
        // loop
        loop {
            println!("again!");
            break;
        }
    }
    ex_loop();
    fn ex_loop_with_break() {
        // loop with break
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is: {result}");
    }
    ex_loop_with_break();

    fn ex_while() {
        // while
        let mut number = 3;
        while number != 0 {
            println!("{}!", number);
            number -= 1;
        }
        println!("LIFTOFF!!!");
    }
    ex_while();

    fn ex_for() {
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
    }
    ex_for();

    fn takes_ownership(some_string: String) {
        // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
    takes_ownership(String::from("hello"));
    // memory is freed.
    fn makes_copy(some_integer: i32) {
        // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
    makes_copy(5);
    fn gives_ownership() -> String {
        // gives_ownership will move its return value into the function that calls it
        let some_string = String::from("hello"); // some_string comes into scope
        some_string // some_string is returned and moves out to the calling function
    }
    gives_ownership();

    fn takes_and_gives_back(a_string: String) -> String {
        // takes_and_gives_back will take a String and return one
        a_string // a_string is returned and moves out to the calling function
    }
    takes_and_gives_back(String::from("hello"));

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
        (s, length)
    }
    calculate_length(String::from("hello"));
    fn ex_ownership() {
        // ownership
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here
        let x = 5; // x comes into scope
        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
        let _s1 = gives_ownership(); // gives_ownership moves its return
                                     // value into s1
        let s2 = String::from("hello"); // s2 comes into scope
        let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
        let s4 = String::from("hello");
        let (s5, len) = calculate_length(s4);
        println!("The length of '{}' is {}.", s5, len);
        // references and borrowing
        let s1 = String::from("hello");
        let _len = calculate_length(s1);
    }
    ex_ownership();
    fn ex_references_and_borrowing() {
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
    ex_references_and_borrowing();
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    fn dangle() -> String {
        // dangle returns a reference to a String
        let s = String::from("hello"); // s is a new String
        s // we return a reference to the String, s
    }
}
