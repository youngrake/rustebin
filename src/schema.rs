table! {
    pastes (id) {
        id -> Nullable<Varchar>,
        owner -> Nullable<Varchar>,
        is_url -> Nullable<Bool>,
        body -> Text,
    }

}

table! {
    users (id) {
        id -> Varchar,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        activated -> Nullable<Bool>,
    }
}

joinable!(pastes -> users (owner));

allow_tables_to_appear_in_same_query!(pastes, users,);
