use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

// 为新的类型实现Serialize
impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 序列化内部的Point2f，作为一个包含x和y的对象
        let mut state = serializer.serialize_struct("Point", 2)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.end()
    }
}

#[derive(Serialize, Debug)]
pub struct CodeInfo {
    pub code: String,
    pub category: String,
    pub points: Vec<Point>,
}
