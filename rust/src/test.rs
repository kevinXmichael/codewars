struct Human {
    age: i32,
    name: String
}

impl Human {
    fn hello(&self) {
        println!("{} ({}) says hello!", self.name, self.age);
    }
}

pub fn run() {
    let kevin = Human { age: 26, name: String::from("Kevin M. Schott") };
    kevin.hello();

    println!("{} __ {}", 'f' as u32, 'g' as u32);
}

