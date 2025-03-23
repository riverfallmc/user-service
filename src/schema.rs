// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "relationship"))]
    pub struct Relationship;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "visibility_enum"))]
    pub struct VisibilityEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Relationship;

    friendships (user_id, friend_id) {
        user_id -> Int4,
        friend_id -> Int4,
        status -> Relationship,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        friends -> Jsonb,
        rank -> Text,
        registered_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::VisibilityEnum;

    users_privacy (id) {
        id -> Int4,
        profile_visibility -> VisibilityEnum,
        friends_visibility -> VisibilityEnum,
        can_invite -> VisibilityEnum,
        server_visibility -> VisibilityEnum,
        online_visibility -> VisibilityEnum,
        hours_visibility -> VisibilityEnum,
    }
}

diesel::joinable!(users_privacy -> users (id));

diesel::allow_tables_to_appear_in_same_query!(
    friendships,
    users,
    users_privacy,
);
