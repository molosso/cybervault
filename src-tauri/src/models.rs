use serde::Serialize;


#[derive(Serialize)]
pub struct VaultItem {

    pub id:i32,

    pub name:String,

    pub login:String,

    pub secret:String

}