# Rust - Studies

## Var

let - immutable variable

let mut - mutable variable

# Data types

### default integer:

i8 (8 bits)
i16 (16 bits)
i32 (32 bits)
i64 (64 bits)
i128 (128 bits)

### ex:

let x: i64 = 23;

### Adding the u means unsigned as the i means signed

### ex:

u8 - 0 e 255

i16 - 32768 e 32767

## Float Point

f32
f64

## Boolean

bool

# Conditional

    let x: u64 = 23;
    let f: u64 = 6;

    if x > f {
        println!("{} > {}", x, f)
    } else {
        println!("{} <= {}", x, f)
    }

# Cast return

    fn function(num: i32) -> f32 {
        num as f32
    }

    fn function(a: f32, b: i128) -> f32 {
        let x = 10.1 as f32 * a + b as f32;
        x
    }

# While

    let mut count = 10;

    while count != 0 {
        count = count - 1;
    }

# For (Not include 10, between 1 and 9) To interate the last number use 1...=10

    for i in 1..10 {
        println!("Number is {}", i)
    }

# Vector

    let animals = vec!["Rabbit", "Cat", "Monkey"];

    for a in animals {
        println!("The animal is {}", a)
    }
