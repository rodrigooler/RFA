#[derive(juniper::GraphQLObject)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
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
pub struct DB {
    pub brazil: Vec<Brazil>
}
