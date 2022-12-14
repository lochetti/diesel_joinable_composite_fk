// @generated automatically by Diesel CLI.

diesel::table! {
    payment_card (id) {
        id -> Int4,
        code -> Text,
    }
}

diesel::table! {
    transaction_one (id) {
        id -> Int4,
        card_code -> Text,
        payment_card_id -> Int4,
    }
}

diesel::table! {
    transaction_two (id) {
        id -> Int4,
        payment_card_id -> Int4,
        card_code -> Text,
    }
}

diesel::joinable!(transaction_one -> payment_card (card_code));
diesel::joinable!(transaction_two -> payment_card (payment_card_id));

diesel::allow_tables_to_appear_in_same_query!(
    payment_card,
    transaction_one,
    transaction_two,
);
