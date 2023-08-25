// @generated automatically by Diesel CLI.

diesel::table! {
    earthquake_events (id) {
        id -> Int4,
        mag -> Float8,
        place -> Text,
        time -> Nullable<Timestamptz>,
        updated -> Nullable<Timestamptz>,
        tsunami -> Int4,
        mag_type -> Text,
        event_type -> Text,
        lon -> Float8,
        lat -> Float8,
    }
}
