pub mod http;

#[derive(Debug)]
pub struct Exception {
    pub code: i32,
    pub message: String,
}

impl Exception {
    #[inline]
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    #[inline]
    fn to_string(&self) -> String {
        format!("{:?}, {}", self.code, self.message)
    }
}