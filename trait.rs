fn main() {
    let name =String::from("Bird");
    let b= Bird{name,attack:44};
    b.print_name();
    println!("{} {}",b.can_fly(), b.is_animal());
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
impl Animal for Bird{
    fn can_fly(&self)->bool{
        true
    }
    fn is_animal(&self)->bool {
        false
    }

}

trait Animal {
    fn can_fly(&self)->bool;
    fn is_animal(&self)->bool {
        true
    }

}

//Rust doesnot support inheritance but support interferance 

