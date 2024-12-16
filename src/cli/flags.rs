pub struct GetFlags {
    pub version: Option<u32>,
}

impl Default for GetFlags {
    fn default() -> Self {
        GetFlags { version: None }
    }
}

impl GetFlags {
    pub fn new() -> Self {
        GetFlags::default()
    }
}
