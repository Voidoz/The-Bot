use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Config {

    #[diesel(sql_type = Integer)]
    pub id: i32,

    #[diesel(sql_type = Text)]
    pub token: String,

    #[diesel(sql_type = Text)]
    pub dev_channel: String
}