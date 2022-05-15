/*
*trait中的Self和self
* Self代表了当前实现了Trait的某个具体的类型本身，比如 File实现了Read，那么Self就代表File
* self用作方法的第一个参数，是self: Self的缩写。
*/ 
use std::fmt;
use std::io::Write;
use std::str::FromStr;
use regex::Regex;
struct BufBuilder{
    buf: Vec<u8>
}

impl  BufBuilder {
    pub fn new()->Self{
        Self{
            buf: Vec::with_capacity(1024)
        }
    }
}

impl fmt::Debug for BufBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{}", String::from_utf8_lossy(&self.buf)) 
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(self.buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // 由于是在内存中操作，所以不需要 flush
        Ok(())
    }
}

// pub trait Parse{
//     fn parse(s: &str)-> Self;
// }

// impl Parse for u8{
//     fn parse(s: &str)->Self{
//         let re: Regex = Regex::new(r"^[0-9]+").unwrap();
//         if let Some(captures) =re.captures(s){
//             captures.get(0).map_or(0,|s| s.as_str().parse().unwrap_or(0))
//         }else{
//             0
//         }
//     }
// }

// impl Parse for f32{
//     fn parse(s: &str)->Self{
//         let re: Regex = Regex::new(r"^[0-9]+\.[0-9]+").unwrap();
//         if let Some(captures) =re.captures(s){
//             captures.get(0).map_or(0.0,|s| s.as_str().parse().unwrap_or(0.0))
//         }else{
//             0.0
//         }
//     }
// }

// pub trait Parse<T>{
//     fn parse(s: &str)->Self;
// }
/*
* 重构第一版： 使用泛型T进行重构
* T 必须可以被str::parse处理
* str::parse是一个泛型函数，返回任何实现了FromStr trait的类型
*/ 

// impl<T> Parse<T> for T 
// where
// T: FromStr + Default
// {
//     fn parse(s: &str)->Self{
//         let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
//         if let Some(captures) =re.captures(s){
//             captures.get(0).map_or(Self::default(),|s| s.as_str().parse().unwrap_or(Self::default()))
//         }else{
//             Self::default()
//         }
//     }
// }

/*
* 重构第二版： 对于解析出错还是解析成default值做更明确的划分
**/ 
pub trait Parse<T>{
    type Error;
    fn parse(s: &str)->Result<Self, Self::Error>
    where Self: Sized;
}
impl<T> Parse<T> for T 
where
T: FromStr + Default
{
    type Error = String;
    fn parse(s: &str)->Result<T,Self::Error>{
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(captures) =re.captures(s){
            captures.get(0).map_or(Err("failed to capture".to_string()),|s| s.as_str().parse().map_err(|_err|"failed to parse captured string".to_string()))
        }else{
            Err("failed to parse string".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_buf_builder() {
        let mut buf_builder = BufBuilder::new();
        let result = buf_builder.write(&[1,2,3]);
        println!("test_buf_builder result {:?}", result);
        assert_eq!(u8::parse("123hello"),Ok(123));
        assert_eq!(f32::parse("123.35hello"),Ok(123.35));
        assert_eq!(u8::parse("hello123.35"),Err("failed to parse string".into()));
    }
}