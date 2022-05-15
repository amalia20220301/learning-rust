use std::ops::Add;
#[derive(Debug)]
pub struct Complex{
real: f64,
imagine: f64,
}
/*
* 第一版本Add实现， 因为add的第一个参数是self，所以会移动参数的所有权，对于实现了Clone trait的类型不影响，但是对于Complex类型，就不太适用
*/ 
// impl Complex {
//     pub fn new(real: f64, imagine: f64)->Self{
//         Self { real, imagine }
//     }
// }

// impl Add for Complex{
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self {
//         let real = self.real + rhs.real;
//         let imagine = self.imagine + rhs.imagine;
//         Self{
//         real,
//         imagine
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_complex() {
        let c1 = Complex::new(1.0, 1f64);
        let c2 = Complex::new(2 as f64, 3.0);
        println!("Complex {:?}", c1 + c2);
    }
}
