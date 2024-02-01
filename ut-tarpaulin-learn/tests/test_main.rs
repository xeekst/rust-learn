
mod test_main {
    use super::add2;
    #[test]
    fn test_2add2() {
        let a = 1;
        let b = 3;
        let sum = add2(a, b);
        panic!("go");
        assert_eq!(4, sum);
    }
}

pub fn add2(a: i32, b: i32) -> i32 {
    a + b
}
