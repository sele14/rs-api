use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::investments;
use schema::investments::dsl::investments as all_investments;

#[derive(Queryable)]
pub struct Instrument {
    id: i32,
    class: String,
    name: String,
    price: f32,
    quantity: i32
}

pub struct NewInvestment {
    pub class: String,
    pub name: String,
    pub price: f32,
    pub quantity: i32
}