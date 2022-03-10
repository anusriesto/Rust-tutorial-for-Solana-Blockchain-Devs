use std::collections::HashMap;

fn main(){
    let mut map = HashMap::new();

    map.insert(0, "h1");
    map.insert(1, "h2");
    println!("{:?}",map);

    match map.get(&0) {
        Some(str1) => println!("{}",str1),
        _=> println!("Doesn't exist in map"),
    }

    match map.get(&2) {
        Some(str2) => println!("{}",str2),
        _=> println!("Doesn't exist in map"),
    }
    map.remove(&1);
    println!("{:?}",map);


}


