use std::{error::Error, result};

fn main() {
    println!("Jakob der Kaffeholer.");
    let mut jakob: u32 = 42;
    jakob = jakob + 1;
    let mut kosta = Human::new("kosta".into());
    kosta.birthday();
    println!("jakob: {}, kosta: {:?}", jakob, kosta);

    let maybe_string: OptionString = OptionString::Some("Kosta war ungezogen".into());
    match maybe_string {
        OptionString::None => println!("String is None"),
        OptionString::Some(txt) => println!("{}", txt),
    }
}
#[derive(Debug)]
struct Human {
    pub name: String,
    age: usize,
}

impl Human {
    pub fn new(name: String) -> Human {
        Human { name: name, age: 0 }
    }
    pub fn birthday(&mut self) {
        self.age = self.age + 1;
    }
}

enum OptionString {
    None,
    Some(String),
}

fn is_even_uint(number: isize) -> Result<bool, String> {
    if number < 0 {
        return Err("Canno't handle negative integer".into());
    }
    return Ok(number % 2 == 0);
}
