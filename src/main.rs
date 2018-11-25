use std::mem;

const MEANING_OF_LIFE:u8 = 42;// substitui todos os MEANING_OF_LIFE do codigo por 42

static MYSTATIC:u8 = 123; // tem endereço de

fn typefun() {
    println!("The meaning of life is {}", MEANING_OF_LIFE) ;
     println!("MYSTATIC is {}", MYSTATIC) ;
    let a:u8 = 123;
    { 
        let a:u16 = 456;
        println!("inner a = {}",a);
    }
    println!("outer a = {}",a);

    let mut b:u16 = 456;
    println!("b = {}",b);

    b = 789;

    println!("b = {} (new)", b); 

    let d = 12345678;
    println!("d = {} of size {} bytes",d, mem::size_of_val(&d) );


    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {} of size {} with {} bits.", z,size_of_z,  size_of_z * 8); 

    let f = 'b';
    println!("f = {} of size {} bytes",f, mem::size_of_val(&f) );

    let g = 2.5;
    println!("g = {} of size {} bytes",f, mem::size_of_val(&g) );

    let h = 4 > 2;
    println!("h = {} of size {} bytes",h, mem::size_of_val(&h) );
    let b_to_pi = f64::powf(3.0, std::f64::consts::PI);
    println!("b_to_pi = {} of size {} bytes",b_to_pi, mem::size_of_val(&b_to_pi) );
}
fn if_statement() {
    let temp = 25;
    if temp > 30 {
        println!("Really Hot!");
    } else if temp < 10 {
        println!("Really Cold!");
    } else {
        println!("Normal!");
    }
    let day = if temp > 20 {"sunny"} else { "cloudy"};
    println!("The day is {}", day  );
    println!("It is {}",if temp > 20 {"hot"} else {"cold"});
    println!( "It is {}",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else  if temp < 10 {
            "very cold"
        } else {
            "cold"
        }
     );
    
}

fn while_and_loop() {
    let mut x = 1;
    while x < 1000 {
        
        x *=  2;
        if x  == 64 {continue;};
        println!(" x = {}", x);
    }
    let mut y = 1;
    loop {
        y *=  2;
        println!(" y = {}", y);
        if y == 1<<10 {break;};
    }
}

fn for_loop() {
    for x in 1..11 {
        println!("{}", x);
    }
    for (pos, y) in (30..40).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_statement() {
    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1...999  => "unknown",
        _ => "invalid"
    };
    println!("The country with code {} is {}", country_code, country );
}


struct Point {
    x:f64,
    y:f64
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p = Point {
        x: 3.0,
        y: 4.0
    };
    println!("Point p({},{})",p.x, p.y );
    let p2 = Point {
        x: 6.0,
        y: 8.0
    };
    let l = Line {
        start: p,
        end: p2
    };
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), // tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8 } //Struct
}

fn enums() {
    let c:Color = Color::CmykColor{cyan:0 , magenta: 128, yellow:0 , black:255};
    match c {
        Color::Red =>  println!("{}","r" ),
        Color::Green => println!("{}","g" ),
        Color::Blue => println!("{}", "b"),
        Color::RgbColor(0,0,0) => println!("{}","black" ),
        Color::RgbColor(r,g,b) => println!("r = {}, g = {}, b = {}",r,g,b),
        Color::CmykColor{cyan:_ , magenta:_, yellow:_ , black:255} => println!("black"),
        _ => println!("{}","qq coisa" )
    }
}

union IntOrFloat {
    i:i32,
    f:f32
}

fn process_value(iof: IntOrFloat){
    unsafe {
        match iof {
            IntOrFloat { i:42 } => println!("meaning of life" ),
            IntOrFloat {f}      => println!("f32 = {}",f )
        }
    }
}

fn unions() {
    let mut iof = IntOrFloat{ i: 123};
    unsafe { iof.i = 42};

    let value = unsafe {iof.i};
    process_value(iof);
    process_value(IntOrFloat{ f: 12.567});
}

fn options() {
    let x = 3.0 ;
    let y =  2.0 ;

    let result:Option<f64> = if y != 0.0 { Some(x / y)} else { None } ;
    println!("{:?}", result );

    match result {
        Some(z) => println!("z = {}", z),
        None => println!("Cannot Divide")
    }
    //if let / while let
    if let Some(z) = result {println!("z = {} (let)", z)};
}

fn arrays() {
    let mut a:[i32; 5] = [1,2,3,4,5];
    println!("a has {} elements, first is {}",  a.len(), a[0]);
    a[0] = 326;
    println!("first is now: {}",a[0] );
    println!("{:?}", a);
    if a == [326,2,3,4,5] {
        println!("match" );
    }
    let b = [1u8; 10] ; // b.len() == 0
    for i in 0..b.len() {
        println!("{}", b[i]);
    };
    println!("b take size of {} bytes", std::mem::size_of_val(&b) );
    let mtx:[[f32;3]; 2] = [
        [1.0, 0.0 , 0.0],
        [0.0, 1.0 , 0.0]
    ];
    println!("mtx = {:?}",mtx );

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {println!("mtx[{}; {}] = {}",i, j, mtx[i][j] )};
        }
    }
}

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);
    let idx:usize = 0;
    a[idx] = 212;
    println!("a[0] = {}", a[idx]);
    match a.get(3) {
        Some(x) => println!("a[3] = {}", x),
        None => println!("error none")
    } 
    for x in &a {println!("{}",x);}

    a.push(77);
    println!("{:?}", a);

    //some(77)
    let last_elem = a.pop();
    println!("last elem is {:?}, a= {:?}", last_elem, a );

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn use_slice(slice: &mut[i32]){
    println!("first elem {}, len ={}", slice[0], slice.len() );
    slice[0]=3321
}

fn slices() {
    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]);
    use_slice(&mut data);
    println!("{:?}",data);
}

fn strings() {
    let s:&'static str = "Hello There!"; // string slice
    for c in s.chars().rev() {
        println!("{}", c);
    }
    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter: {}", first_char );
    }
    // heap String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // &str <> String
    let u:&str = &letters;

    // concatenation
    // string + str
    // let z = letters + & letters

    //let mut abc = String::from("Hello World"); // or 
    let mut abc = "Hello Word".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}

fn sum_and_product(x: i32, y:i32) -> (i32, i32) {
 (x+y, x*y)
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}",sp );
    println!("({}, {})", sp.0, sp.1 );

    // destructuring
    let (a, b) = sp;

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}",combined );
    println!("last elem = {}",(combined.1).1 );

    let ((a,b),(c,d)) = combined;

    let foo = (true, 4.0, -1i8);
    println!("foo = {:?}", foo );

    let meaning = (42,);
    println!("meaning = {:?}", meaning);
}

fn how_many(x:i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        z @ 9...11 => "lots of",
        _ if (x % 2 == 0) => "some",
        _ => "a few"
    }
} 
fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges.", x ,how_many(x));
    }

    let point = (0, 4);
    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!(" x axix, x = {}", x ),
        (0, y) => println!(" y axis; y = {}", y ),
        (x, y) => println!("({}, {})",x, y),
    }
}
struct Point2<T> {
    x: T,
    y: T
} 
struct Point3<T,V> {
    x: T,
    y: V
}
fn generics() {
   let a = Point2 {x:0, y:0};
   let b = Point2 {x: 1.2, y:3.4};
   let c:Point2<u16> =  Point2 {x:0, y:4};
   let d:Point2<f64> = Point2 {x: 1.2, y:3.4};

   let e:Point3<u16,i32> = Point3 {x: 0, y:4};
}
fn print_value(x: i32){
    println!("value = {}", x);
}
fn increase(x: &mut i32){
    *x += 1;
}
fn product(x: i32, y: i32) -> i32 {
    x * y
}
fn functions() {
    print_value(33);

    let mut z = 1;
    increase(&mut z);

    println!("z = {}", z );

    let a = 2;
    let b = 3;
    let p = product(a, b);
    println!(" {} x {} = {}", a , b, p)
}
struct Point4 {
    x: f64,
    y: f64
}
struct Line4 {
    start: Point4,
    end: Point4
}
impl Line4 {
    fn len(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }
}
fn methods() {
    let p = Point4 { x: 3.0, y: 4.0};
    let p2 = Point4 {x: 5.0, y: 10.0};
    let my_line = Line4 {start: p, end: p2};
    println!("myLine.len() = {}", my_line.len() );
}

fn closures() {
    let plus_one = |x:i32| -> i32 { x + 1};
    let a = 6;
    println!("{} + 1 = {}",a, plus_one(a) );
}
fn main() {
    //typefun();
    //if_statement();
    //while_and_loop()
    //for_loop();
    //match_statement(); 
    //structures();
    //enums();
    //unions();
    //options();
    //arrays();
    //vectors();
    //slices();
   // strings();
   //tuples();
   //pattern_matching() ;
  // generics();
  //functions();
  //methods();
  closures();
}