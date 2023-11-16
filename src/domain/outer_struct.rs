use super::inner_struct::InnerStruct;

pub struct OuterStruct {
    val: InnerStruct
}

impl OuterStruct {
    pub fn new(id: u8) -> Self {
        OuterStruct { val: InnerStruct::new(id) }
    }

    pub fn update(&mut self, id: u8) {
        self.val.update_id(id)
    }

    pub fn show(&self) {
        println!("id: {}", self.val.get_id())
    }
}