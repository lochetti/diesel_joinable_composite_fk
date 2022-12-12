// @generated automatically by Diesel CLI.

diesel::table! {
    payment_card (id) {
        id -> Int4,
        code -> Text,
        holder_id -> Int4,
    }
}

diesel::table! {
    person (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    transaction (id) {
        id -> Int4,
        executed_at -> Timestamptz,
        payment_card_id -> Int4,
        card_code -> Text,
    }
}

diesel::joinable!(payment_card -> person (holder_id));
diesel::joinable!(transaction -> payment_card (payment_card_id));

diesel::allow_tables_to_appear_in_same_query!(
    payment_card,
    person,
    transaction,
);
