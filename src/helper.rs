#[derive(Debug, Clone)]
pub struct DataView {
    pub image: Vec<String>,
    pub class_dump: Vec<ClassView>,
}
impl DataView {
    pub fn new() -> Self {
        Self {
            image: Vec::new(),
            class_dump: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassView {
    pub name: String,
    pub fields: Vec<String>,
    pub methods: Vec<String>,
}

impl ClassView {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            fields: Vec::new(),
            methods: Vec::new(),
        }
    }
}
