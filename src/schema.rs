diesel::table! {
    investments (id) {
        id -> Int4,
        class -> Varchar,
        instrument_name -> Varchar,
        price -> Float8,
        quantity -> Int4,
    }
}
