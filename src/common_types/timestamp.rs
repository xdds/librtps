#[derive(PartialEq,Debug,Deserialize)]
pub struct Timestamp {
    pub seconds: u32,
    pub fraction: u32
}