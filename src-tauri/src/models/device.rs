use serde::Serialize;

#[derive(Clone,Debug,Serialize)]
pub struct Device {
    pub id: usize,
    pub name: String,
    pub ip:String,
    pub online:bool,
}