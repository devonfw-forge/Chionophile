table! {
    access_codes (id) {
        id -> Text,
        ticket_number -> Nullable<Varchar>,
        creation_time -> Nullable<Timestamp>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        queue_id -> Nullable<Text>,
        visitor_id -> Nullable<Text>,
    }
}

table! {
    queues (id) {
        id -> Text,
        name -> Nullable<Varchar>,
        logo -> Nullable<Varchar>,
        current_number -> Nullable<Varchar>,
        attention_time -> Nullable<Timestamp>,
        min_attention_time -> Timestamp,
        active -> Bool,
        customers -> Int4,
    }
}

table! {
    visitors (id) {
        id -> Text,
        username -> Varchar,
        name -> Varchar,
        phone_number -> Varchar,
        password -> Nullable<Varchar>,
        accepted_commercial -> Nullable<Bool>,
        accepted_terms -> Nullable<Bool>,
        user_type -> Nullable<Bool>,
    }
}

joinable!(access_codes -> queues (queue_id));
joinable!(access_codes -> visitors (visitor_id));

allow_tables_to_appear_in_same_query!(
    access_codes,
    queues,
    visitors,
);
