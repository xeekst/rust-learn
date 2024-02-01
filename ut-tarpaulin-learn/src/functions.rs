use mockall::automock;

pub fn add3(a: i32, b: i32) -> i32 {
    a + b
}


pub struct DataStruct;
pub struct DataStruct2;

impl DataStruct {
    pub fn new() -> Self {
        DataStruct
    }

    pub fn process_data(&self, id: u32) -> String {
        // 处理数据的逻辑
        format!("Processed data for id {}", id)
    }
}

#[automock]
pub trait DataService {
    fn get_data(&self, id: u32) -> String;
}

#[automock]
impl DataService for DataStruct {
    fn get_data(&self, id: u32) -> String {
        // 在这里调用 DataStruct 的具体方法，这里假设 get_data 方法内部委托给了 process_data 方法
        self.process_data(id)
    }
}

impl DataStruct2 {
    pub fn new() -> Self {
        DataStruct2
    }

    pub fn process_data(&self, id: u32) -> String {
        // 处理数据的逻辑
        format!("Processed data for id2 {}", id)
    }
}

impl DataService for DataStruct2 {
    fn get_data(&self, id: u32) -> String {
        // 在这里调用 DataStruct 的具体方法，这里假设 get_data 方法内部委托给了 process_data 方法
        self.process_data(id)
    }
}