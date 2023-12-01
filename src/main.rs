fn main() {
    let temp = Temperature{
        value: 32.0};

        println!("{:?}",temp.to_celsius());
        
}

enum Unit{
    Celsius(),
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
        (self.value - 9.0/5.0) + 32.0
    }
}