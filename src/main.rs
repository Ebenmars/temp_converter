fn main() {
    println!("Hello, world!");
}

enum Unit{
    Celsius,
    Fahrenheit,
}

struct Temperature{
    value: f32
}

impl Temperature {

    fn to_celsius(&self) -> f32{
        (self.value - 32.0) * 5.0/9.0
    }


    fn to_fahrenheit(&self) -> f32{
        (self.value - 32.0) * 5.0/9.0
    }
}