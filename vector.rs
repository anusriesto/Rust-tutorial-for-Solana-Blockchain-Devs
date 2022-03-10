fn main() {
    let mut ve: Vec<i64>= vec![1,2,3,4,5,6];  //Dynamic array we can remove data 
    // we wish,and they are mutable as well but with mut keyword

    ve.len();
    ve[0];
    ve.push(6);
    ve.remove(0);
    println!("{:?}",ve);


}


