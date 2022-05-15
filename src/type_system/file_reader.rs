use std::fs::File;
use std::io::{BufReader, Read, Result};
/*
* 泛型可以逐步约束， 在实现new方法的时候，没有约束泛型R，
* 在实现process的时候，需要用到R的方法，限制R必须实现Read trait
*/ 
struct MyReader<R>{
    reader: R,
    buf: String,
}

// 
impl<R> MyReader<R> {
    pub fn new(reader: R)->Self{
        Self{
            reader,
            buf: String::with_capacity(1024)
        }
    }
}

impl<R> MyReader<R> where R: Read{
    pub fn process(&mut self) -> Result<usize>{
        self.reader.read_to_string(&mut self.buf)
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use super::*;
    #[test]
    fn test_my_reader() {
        let f = File::open("/etc/hosts").unwrap();
        let mut reader  = MyReader::new(f);
        let result = reader.process();
        println!("result: {:?}",result);
        println!("result: {:?}",reader.buf);
    }
}