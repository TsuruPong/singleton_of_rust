pub struct InnerStruct {
    id: u8
}

impl InnerStruct {
    pub fn new(id: u8) -> Self {
        InnerStruct {
            id
        }
    }

    pub fn update_id(&mut self, id: u8) {
        self.id = id;
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }
}