#[cfg(test)]
mod test_main {
    use crate::add3;

    #[test]
    fn test_add() {
        let a = -1;
        let b = 3;
        let sum = add3(a, b);

        assert_eq!(a + b, sum);

        let a = 11;
        let b = 3;
        let sum = add3(a, b);

        assert_eq!(a - b, sum);
    }
}

#[cfg(test)]
mod tests {
    use crate::functions::MockDataService;
    
    use super::*;
    use crate::functions::{DataService, DataStruct};
    use mockall::predicate::*;
    use mockall::*;

    #[test]
    fn test_data_service() {
        // 创建模拟对象
        let mut mock_data_struct = MockDataService::new();

        let d = DataStruct::new();
        // 定义模拟对象的行为
        mock_data_struct
            .expect_get_data()
            .returning(|id| format!("Processed data for id {}", id));

        // 使用模拟对象进行测试
        assert_eq!(mock_data_struct.get_data(42), d.get_data(42));
    }
}
