// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        age -> Nullable<Int4>,
    }
}
