pub fn main() {
    own_tuple();
    own_array();
    own_struct();
    own_enum();
    own_result();
    own_vec();
    own_box();
    own_variables();
    own_flow_control();
    own_iteration();
    ownership_and_lifetime();
}

fn own_tuple() {
    // Tuple is primitive type. finite heterogeneous sequence.
    println!("# Tuple");
    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";
    println!("{:?}", t);

    fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
        (t, s)
    }
    let t1 = make_tuple(1, 2);
    let t2 = make_tuple("Make", "Tuple");
    let t3 = make_tuple(vec![1, 2, 3], vec![4, 5]);
    let t4 = make_tuple(3, "years old");
}

fn own_array() {
    // Array is primitive type. fixed-size.
    println!("# Array");
    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    a[1] = b[1];
    a[2] = b[2];
    println!("a[1..3]: {:?}", &a[1..3]);
    println!("a: {:?}", a);
}

fn own_struct() {
    println!("# Struct");
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32
    }

    let p = Person {
        name: String::from("John"),
        age: 8
    };

    println!("{:?}", p);
}

fn own_enum() {
    println!("# Enum");
    #[derive(Debug)]
    enum Event {
        Quit,
        KeyDown(u8),
        MouseDown {x: i32, y: i32}
    }

    let e1 = Event::Quit;
    let e2 = Event::MouseDown { x: 10, y: 10};

    println!("{:?} {:?}", e1, e2);
}

fn own_result() {
    println!("# Result");
    let result: Result<i32, String> = Ok(200);

    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err)
    }

    let result: Result<i32, String> = Ok(200);
    println!("code: {}", result.unwrap_or(-1));
    let result: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result.unwrap_or(-1));

    fn func(code: i32) -> Result<i32, String> {
        println!("code: {}", code);
        Ok(100)
    }

    let result: Result<i32, String> = Ok(200);
    let next_result = result.and_then(func); // it call func()
    let result: Result<i32, String> = Err("error".to_string());
    let next_result = result.and_then(func); // it doesn't call func()

    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        let code = result?;
        println!("error_handler - code: {}", code);
        Ok(100)
    }
    let result: Result<i32, String> = Ok(200);
    error_handling(result).expect("TODO: panic message");
    let result: Result<i32, String> = Err("error".to_string());
    // error_handling(result).expect("TODO: panic message"); // it raise panic
}

fn own_vec() {
    println!("# Vector");
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0; 5];
    println!("{:?}", v1);
    println!("{:?}", v2);
    let v = vec![1, 2, 3, 4, 5];
    for element in &v {
        println!("{}", element);
    }
}

fn own_box() {
    println!("# Box");
    // It will be return compile error. cause rust compiler doesn't know array size at compile time.
    // let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    // print(byte_array);
    //
    // fn print(s: [u8]) {
    //     println!("{:?}", s)
    // }
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    print(Box::new(byte_array));
    let byte_array = [b'w', b'o', b'r', b'l', b'd', b'!'];
    print(Box::new(byte_array));

    fn print(s: Box<[u8]>) {
        println!("{:?}", s);
    }
}

fn own_variables() {
    println!("# Variables");
    let immut_val = 10;
    let mut mut_val = 20;

    //immut_val += mut_val; // Cannot assign twice to immutable variable
    mut_val += immut_val;
    println!("{}", mut_val);

    let v1: u64 = 10;
    let v2 = 10u64;
}

fn own_flow_control() {
    println!("# Flow control");
    let number = 1;
    if 0 < number {
        println!("0 < number");
    } else if number < 0 {
        println!("number < 0");
    } else {
        println!("0 == number");
    }

    // Rust is an expression-oriented language. so...
    let number = 1;
    let result = if 0 <= number {
        number
    } else {
        -number
    };
    println!("{}", number);

    let mut count = 0;
    let result = loop {
        println!("Count in loop: {}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };

    let mut count = 0;
    while count < 10 {
        println!("Count in while: {}", count);
        count += 1;
    }

    let count: i32;
    for count in 0..10 {
        println!("Count in for: {}", count);
    }

    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for element in &array {
        println!("element: {}", element);
    }

    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");
            break 'main;
            println!("sub loop end"); // unreachable code
        }
        println!("main loop end") //unreachable code
    }

    let i: i32 = 1;
    match i {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("misc")
    }

    enum Color {
        Red,
        Blue,
        Green
    }
    let c = Color::Blue;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green")
    }

    let result: Result<i32, String> = Ok(100);
    let result_number = match result {
        Ok(number) => number,
        Err(message) => {
            println!("Error: {}", message);
            -1
        }
    };
}

fn own_iteration() {
    println!("# Iteration");
    for number in 1..5 {
        println!("iter for: {}", number);
    }

    let it = Iter {
        current: 0,
        max: 10
    };
    for num in it {
        println!("impl Iterator: {}", num);
    }
    struct Iter {
        current: usize,
        max: usize
    }
    impl Iterator for Iter {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            self.current += 1;
            if self.current - 1 < self.max {
                Some(self.current - 1)
            } else {
                None
            }
        }
    }
}

fn ownership_and_lifetime() {
    struct Color {
        r: i32,
        g: i32,
        b: i32
    }
    let a = Color{r:255, g:255, b:255};
    let b = a; // owner changed
    println!("{} {} {}", b.r, b.g, b.b);

    let mut important_data = "I'm important".to_string();
    important_data = calc_data(important_data);
    println!("{}", important_data);
    fn calc_data(data: String) -> String {
        println!("call data from calc_data: {}", data);
        data
    }

    let important_data = "I'm important".to_string();
    calc_data_with_ref(&important_data);
    println!("by ref: {}", &important_data);
    fn calc_data_with_ref(data: &String) {
        println!("calc_data_with_ref: {}", data);
    }

    let x = 5;
    let y = &x; // immutable
    let z = &x; // immutable
    dbg!(x);
    dbg!(y);
    dbg!(z);

    let mut x = 5;
    {
        let y = &mut x; // mutable
        //let z = &mut x; // mutable with error: second mutable borrow occurs here
        dbg!(y);
        //dbg!(z);
    }

    {
        let y = &x; // immutable
        // let z = &mut x; // mutable with error: mutable borrow occurs here
        dbg!(y);
        //dbg!(z);
    }

    // let y;
    {
        let x = 5;
        //y = &x; // borrowed value does not live long enough
        dbg!(x);
    }
    // dbg!(y);

    let mut x = 5;
    let y = &x;
    // let z = &mut x; // mutable borrow occurs here
    dbg!(y);
    dbg!(x);

    let mut x = 5;
    let y = &x;
    let z = &mut x;
    dbg!(z);
    dbg!(x);

    // RAII
    struct Droppable;

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("Resource will be released!");
        }
    }
    {
        let d = Droppable;
    }
    println!("The Droppable should be released at the end of block.");
}