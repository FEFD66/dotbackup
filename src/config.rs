use serde::{Serialize,Deserialize};
use serde_json::Result;

#[derive(Deserialize,Serialize,Debug)]
pub struct DotConfig{
    pub general:GeneralConfig,
    pub groups:Vec<Group>,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct GeneralConfig{
    pub compress:Option<Compress>,
    pub back_cmd:String,
    pub back_root:String,
}
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Group{
    pub compress:Option<Compress>,
    pub name:String,
    pub with_root:bool,
    pub items:Vec<Item>,
}
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Item{
    pub compress:Option<Compress>,
    pub paths:String,
}
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Compress{
    pub r#type:String,
    pub args:Vec<String>,
}