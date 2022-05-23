fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }

    None
}

fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_pos() {
        let data = vec![10, 42, 9, 8];
        let v = 42;

        if let Some(pos) = find_pos(data, v) {
            println!("Found {} at {}", v, pos);
        }
        /*
         * 数据在参数传递，变量赋值，函数返回值这些行为中，默认会发生所有权的转移，但是：
         * 1. 如果数据实现了COPY，则会进行浅拷贝，也就是按位copy。
         * 原生数据类型： 函数，不可变引用和裸指针实现了Copy
         * 数组和元祖，如果其内部的数据结构实现了Copy， 那么它们也实现了Copy
         * 可变引用没有实现Copy
         * 非固定大小的数据结构，没有实现Copy
         * 2. 可以使用borrow。
         */
        // println!("Check value existence {:?} at {}", data, v);
    }

    #[test]
    fn test_find_pos_2() {
        let data = vec![1, 2, 3, 4];
        let data1 = data;
        println!("sum of data1: {}", sum(data1));
        // println!("data1: {:?}", data1); // 报错， data1的所有权已经move到sum函数内部
        // println!("sum of data: {}", sum(data)); // 报错， data的所有权已经move到data1
    }
}
