use std::collections::BTreeMap;
fn type_system(){
    let x = 32;
    let mut y =64;
    let raw = &x as *const i32;
    let raw_mut = &mut y as *mut i32;
    let boxed: Box<[i32]> = Box::new([1,2,3]);
    let mut bt_map = BTreeMap::new();
    bt_map.insert("hello","word");
    println!("bt_map {:?}", bt_map);
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // collect 是 iterator trait的方法，很多类型都实现了iterator trait，所以对于collect，必须显示地声明返回类型。
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|x| x%2==0).collect();
    println!("even_numbers {:?}", even_numbers);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_type_system() {
        type_system();
    }
}