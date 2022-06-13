// pub struct Service<Store=MemTable>{
//     inner: Arc<ServiceInner<Store>>
// }
// 在实现的过程中逐步约束
// impl<Store> Service<Store>{
//     pub fn new(store: Store) -> Self{
//
//     }
// }
// impl<Store: Storage> Service<Store>{
//     pub fn execute(&self, cmd: CommandRequest) -> CommandResponse{
//
//     }
// }


// user和product都有id（u64）字段，但是id只能和同类型的id比较。比如user和product的id比较就会报错。

use std::marker::PhantomData;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Identifier<T> {
    inner: u64,
    // 在定义数据结构时，对于额外的，暂时不需要的泛型参数，用PhantomData来"拥有"， 可以规避编译时报错。
    _tag: PhantomData<T>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct User {
    id: Identifier<Self>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Product {
    id: Identifier<Self>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_should_not_be_the_same() {
        let user = User::default();
        let product = Product::default();

        // 两个 id 不能比较，因为他们属于不同的类型
        // assert_ne!(user.id, product.id);

        assert_eq!(user.id.inner, product.id.inner);
    }
}