fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}

// fn local_ref<'a>() -> &'a i32 {
//     let a = 42;
//     //报错，引用了函数内部的变量，离开函数的作用域，就会成为悬垂引用
//     &a
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum() {
        let data = vec![1, 2, 3, 4];
        let data1 = &data;
        // data1, &data, 以及传递给sum的参数data1'， 都是指向了堆上面的data本身，但是它们引用的地址是不同的。
        println!("addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}", &data, data1, &&data, &data1);
        println!("sum of data1: {}", sum(data1));
        // 堆上数据的地址是什么
        println!("addr of items: [{:p}, {:p}, {:p}, {:p}]", &data[0], &data[1], &data[2], &data[3]);
    }

    // #[test]
    // fn test_local_ref(){
    //     let r = local_ref();
    //     println!("r: {:p}", r);
    // }

    #[test]
    fn test_vec() {
        let mut data: Vec<&u32> = Vec::new();
        let v = 42;
        data.push(&v);
        println!("data: {:?}", data);
    }

    #[test]
    fn test_multiple_ref() {
        let mut data = vec![1, 2, 3];
        let data1 = vec![&data[0]]; // data1 持有了data中数据的不可变引用
        println!("data[0]: {:p}", &data[0]);
        // for i in 0..100 {
        //     data.push(i); //data的可变引用，不安全，如果data在堆上的地址重新分配了，就会导致data1对data的不可变引用失效。
        // }
        println!("data[0]: {:p}", &data[0]); //data的不可变引用
        println!("boxed: {:p}", &data1); // data1的不可变引用
    }
    #[test]
    fn test_mut_borrow(){
        let mut data = vec![1,2,3,4];
        let b = &mut data;
        // 可变引用没有实现Copy, 为何不报错？？？
        println!("sum of data {}", sum(b));
        println!("b:  {:?}", b);
    }
    #[test]
    fn test_mut_borrow_1(){
        let mut data = vec![1,2,3,4];
        // 采用解引用的方式，隐含使用了i32的Copy Trait，让last Copy 了一份。
        let last = *data.last().unwrap();
        data.push(5);
        println!("data: {:?}", data);

        // let mut arr = vec![String::from("a"), String::from("b")];
        // // String 会做move，所以不适用。
        // let last = *arr.last().unwrap();
        // arr.push(String::from("c"));
        // println!("data: {:?}", last);
    }
}