mod type_system;
mod ownership;
mod borrow;
mod data_structure;
mod example;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
