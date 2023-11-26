table! {
    dogs (id) {
        id -> Nullable<Integer>,
        breed -> Varchar,
        color -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(dogs,);