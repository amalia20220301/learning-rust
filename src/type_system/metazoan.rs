/*
*trait中的Self和self
* Self代表了当前实现了Trait的某个具体的类型本身，比如 File实现了Read，那么Self就代表File
* self用作方法的第一个参数，是self: Self的缩写。
*/ 
use std::fmt;
use std::io::Write;
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

pub trait Parse{
    fn parse(s: &str)-> Self;
}

impl Parse for u8{
    fn parse(s: &str)->Self{
        let re: Regex = Regex::new(r"^[0-9]+").unwrap();
        if let Some(captures) =re.captures(s){
            captures.get(0).map_or(0,|s| s.as_str().parse().unwrap_or(0))
        }else{
            0
        }
    }
}

impl Parse for f32{
    fn parse(s: &str)->Self{
        let re: Regex = Regex::new(r"^[0-9]+\.[0-9]+").unwrap();
        if let Some(captures) =re.captures(s){
            captures.get(0).map_or(0.0,|s| s.as_str().parse().unwrap_or(0.0))
        }else{
            0.0
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
        assert_eq!(u8::parse("123hello"),123);
        assert_eq!(f32::parse("123.35hello"),123.35);
    }
}