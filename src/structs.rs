#[derive(Serialize, Deserialize)]
pub struct Brazil {
    pub id: String,
    pub city: String,
    pub country: String,
    pub neighborhood: String,
    pub state: String,
    pub street: String,
    pub uf: String
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub brazil: Vec<Brazil>
}