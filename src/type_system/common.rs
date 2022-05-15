use std::collections::BTreeMap;
use std::net::SocketAddr;
/*
* 常量必须指定其变量类型
*/ 
const PI: f32 =3.1415;

fn type_system(){
    let x = 32;
    let mut y =64;
    let raw = &x as *const i32;
    let raw_mut = &mut y as *mut i32;
    let boxed: Box<[i32]> = Box::new([1,2,3]);
    let mut bt_map = BTreeMap::new();
    bt_map.insert("hello","word");
    println!("bt_map {:?}", bt_map);
    /*
     * collect 是 iterator trait的方法，很多类型都实现了iterator trait，所以对于collect，必须显示地声明返回类型。
     * 编译器无法推断出集合的类型，但是可以推断出集合内部元素的类型，所以even_numbers的类型可以声明为Vec<_>
     *  */
    let even_numbers: Vec<_> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter().filter(|x| x%2==0).collect();
    /*
    * 也可以给collect指定返回值的类型
    */ 
    let even_numbers_2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter().filter(|x| x%2==0).collect::<Vec<_>>();
    println!("even_numbers {:?}", even_numbers);
    println!("even_numbers {:?}", even_numbers_2);
    /*
    *ip地址和端口转换
     * */ 
    let addr="127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    println!("addr: ip: {:?}, port: {:?}", addr.ip(),addr.port());
}

/*
* ?sized ?代表放松后面的约束。?sized代表可变大小。因为rust里面，泛型默认的参数都需要是sized。
* toOwned: 是一个trait，可以把借用的数据clone出一个拥有所有权的数据。
**/

pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned,
{
    // 借用的数据
    Borrowed(&'a B),
    // 拥有的数据
    Owned(<B as ToOwned>::Owned),
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_type_system() {
        type_system();
    }
}