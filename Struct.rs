fn main() {
    let name =String::from("Bird");
    let b= Bird{name,attack:44};
    b.print_name();
}

struct Bird { 
    name: String,
    attack : u64
    
}

impl Bird{
    fn print_name(&self) {
        println!("{}",self.name);

    }
}



