// fn main() {
    //function
    //println!("{}", is_even(11));

//}
//pub fn is_even(num:u8) -> bool {
    //let digit:u8 = num %2;
    //digit== 0 // return value of a function thats why no semicolon at the end
//}

//fn main() {
    //let mut num=5;
    //num=3;
    //println!("{}",num);
//}

fn main() {
    let arr=[0,1,2,3,4,5]; //array is done
    let slice= &arr[1 .. 4];  //1 is inclusive but 3 is exclusive index number
    burrowing_slices(arr,slice);
}

fn burrowing_slices(arr: [u8; 6], slice: &[u8]) {
    println!("{:?}",arr);
    println!("{:?}",slice);
    println!("length: {}",slice.len());
    println!("{} {}",slice[0],slice[1]);


}

