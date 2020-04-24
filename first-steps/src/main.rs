fn main() {	
    println!("Hello, world!!");

    let x = 42;

    let _y: i32 = 32;
    //Temos i8, i16, i32, i64, i128
    //Também u8, u16, u32, u64, u128 para unsigned

    //Para utilizar uma variável é preciso inicializa-la antes

    let _x = x + 3;
    //O primeiro 'x' não existe mais. 
    
    let _pair = ('a', 17);
    let _pair: (char, i32) = ('a', 17);

    let (_some_char, _some_int) = ('a', 17);

    //let (_, right) = slice.split_at(middle)

    let _x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |_x, y| y + y);


    greet();
    //fair_dice_roll();

    {
        let _x = "in";
    }

    let a = (10, 20);
    a.0;

    let nick = "fafafafafaf";
    nick.len();


    let _least = std::cmp::min(3, 8); //resp = 3

    //crate(library)::module(source file)::function
    use std::cmp::min; //"traz para o escopo"
    //use std::cmp::max;
    
    let _least = min(7, 1);

    //use std::cmp::{min, max};
    //use std::{cmp::min, cmp::max};

    //Import all
    //use std::cmp::*;

    let _x = "amos".len();
    let _x = str::len("amos");
    
    //Rust insere 'use std::prelude::v1::*;' no inicio de cada modulo

    struct Vec2{
        x: f64, //Float com 64-bits
        y: f64,
    }

    let v1 = Vec2{x: 1.0, y: 3.0};

    let v2 = Vec2{
        x: 14.0,
        ..v1
    };

    let _v3 = Vec2{ ..v2};

    //let (left, right) = slice.split(v3);

    let v4 = Vec2{ x: 3.0, y: 6.0};
    let Vec2{x: _, y: _} = v4;
    //let Vec2 {x, ..} = v4; //Isso discarta o v4.y

    let one = Number {odd: true, value: 1};
    let two = Number {odd: false, value: 2};
    print_number(one);
    print_number(two);   



}

fn print_number(n: Number){
    match n{
        Number {odd: true, value} => println!("{} é impar!", value),
        Number {odd: false, value} => println!("{} é par!", value)
    }
}
// fn print_number(n: Number){
//     if let Number {odd: true, value} = n {
//         println!("Impar: {}", value);
//     } else if let Number {odd: false, value} = n {
//         println!("Par: {}", value);
//     }
// }
struct Number{
    odd: bool,
    value: i32,
}
// fn fair_dice_roll() -> i32{
//     if feeling_luck {
//         6
//     } else {    
//         4 //omitir o ';' no final da função é a mesma coisa que retornar
//     }
// }

// fn fair_die_roll() -> i32{
//     match feeling_luck {
//         true => 6,
//         false => 4,
//     }
// }

fn greet(){
    println!("Hei there!");
}

