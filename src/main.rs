use juniper::{FieldResult};

#[derive(juniper::GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

struct Context {
    // pool: DatabasePool,
}

impl juniper::Context for Context {}

struct Query;

#[juniper::object(
    Context = Context,
)]

impl Query {

    fn apiVersion() -> &str {
        "1.0"
    }

    fn human(context: &Context, id: String) -> FieldResult<Human> {
        let connection = context.pool.get_connection()?;
        let human = connection.find_human(&id)?;
        Ok(human)
    }
}

// Now, we do the same for our Mutation type.

struct Mutation;

#[juniper::object(
    Context = Context,
)]

impl Mutation {}

type Schema = juniper::RootNode<'static, Query, Mutation>;