use std::mem;
use std::ops::Deref;

pub enum Cow<'a, B> where B: 'a + ToOwned + ?Sized {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}

pub trait Borrow<Borrowed> where Borrowed: ?Sized {
    fn borrow(&self) -> &Borrowed;
}

pub trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;
    fn clone_into(&self, target: &mut Self::Owned);
}

// impl ToOwned for str {
//     // String作为关联类型，必须实现Borrow<str>
//     type Owned = String;
//     #[inline]
//     fn to_owned(&self) -> String {
//         unsafe { String::from_utf8_unchecked(self.as_bytes().to_owned()) }
//     }
//     fn clone_into(&self, target: &mut String) {
//         let mut b = mem::take(target).into_bytes();
//         self.as_bytes().clone_into(&mut b);
//         *target = unsafe { String::from_utf8_unchecked(b) }
//     }
// }
//
// impl Borrow<str> for String {
//     fn borrow(&self) -> &str {
//         &self[..]
//     }
// }
//
// // 因为Cow是智能指针，所以自然需要实现Deref
//
// impl<B: ToOwned + ?Sized> Deref for Cow<'_, B> {
//     type Target = B;
//
//     fn deref(&self) -> &Self::Target {
//         match *self {
//             Borrowed(borrowed) => borrowed,
//             Owned(ref owned) => owned.borrow()
//         }
//     }
// }

#[cfg(test)]
mod test {
    use std::borrow::Cow;
    use url::Url;

    #[test]
    fn test_cow() {
        let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
        let mut pairs = url.query_pairs();
        assert_eq!(pairs.count(), 3);
        // 因为k，v都是cow<str>, 用起来跟&str 或者String都一样。
        let (mut k, v) = pairs.next().unwrap();
        println!("key: {}, v: {}", k, v);
        //当发生修改时，变成Owned
        k.to_mut().push_str("_lala");
        print_pairs((k, v));
        print_pairs(pairs.next().unwrap());
        // 在处理extra=hello%20world时，value被处理成"hello world"
        // 所以，value是owned
        print_pairs(pairs.next().unwrap());
    }

    fn print_pairs(pair: (Cow<str>, Cow<str>)) { println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1)); }

    fn show_cow(cow: Cow<str>) -> String {
        match cow {
            Cow::Borrowed(v) => format!("Borrowed {}", v),
            Cow::Owned(v) => format!("Owned {}", v),
        }
    }
}
