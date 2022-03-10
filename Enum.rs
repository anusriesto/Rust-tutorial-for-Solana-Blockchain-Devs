fn main() {
    let a: Myenum=Myenum::A;
    let b: Myenum=Myenum::B(55);
    let c: Myenum=Myenum::C{x:20,y:59};

    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",c); 
    
    if let Myenum::B(val)=b{
        println!("{}",val);
    }
    if let Myenum::C{x,y}=c{
        println!("{} {}", x, y);

    }

}

#[derive(Debug)]
enum Myenum {
    A,
    B(i32),
    C {x:i32,y:i32}
}

