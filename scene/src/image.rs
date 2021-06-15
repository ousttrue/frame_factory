pub struct Image {
    id: u32,
    pub bytes: Vec<u8>,
    pub name: String,
    pub mime: String,
}

static mut G_NEXT_ID: u32 = 1;

fn next_id() -> u32 {
    unsafe {
        let id = G_NEXT_ID;
        G_NEXT_ID += 1;
        id
    }
}

impl Image {
    pub fn new(bytes: Vec<u8>, name: String, mime: String) -> Image {
        let id = next_id();
        Image {
            id,
            bytes,
            name,
            mime,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
