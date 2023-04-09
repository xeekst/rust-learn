trait Node {
    fn add_child(&mut self, node: Box<dyn Node>);
    fn get_children(&self) -> &Vec<Box<dyn Node>>;
    fn print(&self);
}

struct Folder {
    name: String,
    children: Vec<Box<dyn Node>>,
}

impl Folder {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            children: Vec::new(),
        }
    }
}

impl Node for Folder {
    fn add_child(&mut self, node: Box<dyn Node>) {
        self.children.push(node);
    }

    fn get_children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }

    fn print(&self) {
        println!("Folder: {}", self.name);
    }
}

struct File {
    name: String,
    children: Vec<Box<dyn Node>>,
}

fn get_vector() -> Vec<i32> {
    let v = vec![1, 2, 3];

    v
}

impl File {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            children: Vec::new(),
        }
    }
}

impl Node for File {
    fn add_child(&mut self, _node: Box<dyn Node>) {
        // do nothing
    }

    fn get_children(&self) -> &Vec<Box<dyn Node>> {
        // return empty vector
        &self.children
    }

    fn print(&self) {
        println!("File: {}", self.name);
    }
}

fn main() {
    let mut root = Folder::new("root");

    let mut folder1 = Folder::new("folder1");
    let file1 = File::new("file1");
    let file2 = File::new("file2");
    folder1.add_child(Box::new(file1));
    folder1.add_child(Box::new(file2));
    root.add_child(Box::new(folder1));

    let mut folder2 = Folder::new("folder2");
    let file3 = File::new("file3");
    let file4 = File::new("file4");
    folder2.add_child(Box::new(file3));
    folder2.add_child(Box::new(file4));
    root.add_child(Box::new(folder2));

    root.print();

    for child in root.get_children() {
        child.print();
        for grandchild in child.get_children() {
            grandchild.print();
        }
    }
}
