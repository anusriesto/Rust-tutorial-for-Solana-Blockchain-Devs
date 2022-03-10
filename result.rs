#[derive(Debug)]
enum Myerror {
    Error1
}



fn divide(dividend:i32,divisor:i32) ->Result<i32,Myerror> { //option return two thing NOne and some but results contains
    if dividend % divisor !=0 {  // first one is err,an enum that contains an error code
        Err(Myerror::Error1)                     //Ok(value) A wrapper that contains value
    } else{
        Ok(dividend/divisor)
    }
}
fn main() {
    let divide1: Option<i32> =divide(4,2);
    let divide2: Option<i32> =divide(3,2);

    println!("{:?} unwraps to {}",divide1,divide1.unwrap());
    //println!("{:?} unwraps to {}",divide2,divide2.unwrap()); //it is returning None value thats why error

}



