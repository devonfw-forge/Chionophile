/*
    The schema is the matching that the framework Diesel does with the database. 
    With this, Diesel can transform your entity to its SQL equivalent.
    The sql name annotation is optional, but here it's use in some fields in order to keep Rust's
    naming convention without modifying the Database. In the fields that are equal, it's used for
    consistency.
*/

table! {
    users (id) {
        #[sql_name="id"]
        id -> Int8,
        #[sql_name="username"]
        username -> Nullable<Varchar>,
        #[sql_name="name"]
        name -> Nullable<Varchar>,
        #[sql_name="password"]
        password -> Nullable<Varchar>,
        #[sql_name="phonenumber"]
        phone_number -> Nullable<Varchar>,
        #[sql_name="acceptedcommercial"]
        accepted_commercial -> Nullable<Bool>,
        #[sql_name="acceptedterms"]
        accepted_terms -> Bool,
    }
}
