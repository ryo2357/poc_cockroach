use diesel::prelude::*;

use poc_cockroach::{
    models::{self, NewUser, User},
    schema,
};

fn main() {
    let user = NewUser {
        name: "hoge".to_owned(),
    };
    println!("{:?}", user);

    let conn = &mut models::establish_connection();

    let user = diesel::insert_into(schema::users::table)
        .values(&user)
        // .get_result::<User>(&conn);
        .get_result::<User>(conn);

    println!("{:?}", user);
}
