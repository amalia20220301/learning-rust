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


impl Complex {
    pub fn new(real: f64, imagine: f64)->Self{
        Self { real, imagine }
    }
}

// impl Add for &Complex{
//     type Output = Complex;

//     fn add(self, rhs: Self) -> Self::Output {
//         let real = self.real + rhs.real;
//         let imagine = self.imagine + rhs.imagine;
//         Complex::new(real, imagine)
//     }
// }

/**
 * Add 是一个支持泛型的trait，所以可以为Complex实现与不同类型的Add 操作
pub trait Add<Rhs = Self> {
    type Output;
    #[must_use]
    fn add(self, rhs: Rhs) -> Self::Output;
}
 * **/ 

impl Add<f64> for Complex{
    type Output = Complex;
    fn add(self, rhs: f64) -> Self::Output{
        let real = self.real+rhs;
        Complex::new(real, self.imagine)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_complex() {
        let c1 = Complex::new(1.0, 1f64);
        let c2 = Complex::new(2 as f64, 3.0);
        // println!("Complex {:?}", &c1 + &c2);
        println!("Complex {:?}", c1 + 5.0);
    }
}
