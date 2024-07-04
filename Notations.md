# Rust - Studies

## Var

let - immutable variable

let mut - mutable variable

# Tipos de dados

### inteiro padrão:

i8 (8 bits)
i16 (16 bits)
i32 (32 bits)
i64 (64 bits)
i128 (128 bits)

### ex:

let x: i64 = 23;

### Adicionando o u siginifica unsigned quanto o i significa signed

### ex:

u8 - 0 e 255

i16 - 32768 e 32767

## Float Point

f32
f64

## Boolean

bool

# Condicional

    let x: u64 = 23;
    let f: u64 = 6;

    if x > f {
        println!("{} > {}", x, f)
    } else {
        println!("{} <= {}", x, f)
    }
