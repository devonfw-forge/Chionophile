table! {
    accesscode (id) {
        id -> Int8,
        #[sql_name="modificationcounter"]
        modification_counter -> Int4,
        #[sql_name="creationtime"]
        creation_time -> Nullable<Timestamp>,
        #[sql_name="starttime"]
        start_time -> Nullable<Timestamp>,
        #[sql_name="endtime"]
        end_time -> Nullable<Timestamp>,
        #[sql_name="idvisitor"]
        visitor_id -> Int8,
        #[sql_name="idqueue"]
        queue_id -> Int8,
    }
}

table! {
    dailyqueue (id) {
        id -> Int8,
        #[sql_name="modificationcounter"]
        modification_counter -> Int4,
        name -> Nullable<Varchar>,
        logo -> Nullable<Varchar>,
        #[sql_name="currentnumber"]
        current_number -> Nullable<Varchar>,
        #[sql_name="attentiontime"]
        attention_time -> Nullable<Timestamp>,
        #[sql_name="minattentiontime"]
        min_attention_time -> Timestamp,
        active -> Bool,
    }
}

table! {
    visitor (id) {
        id -> Int8,
        #[sql_name="modificationcounter"]
        modification_counter -> Int4,
        username -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        #[sql_name="phonenumber"]
        phone_number -> Nullable<Varchar>,
        #[sql_name="acceptedcommercial"]
        accepted_commercial -> Nullable<Bool>,
        #[sql_name="acceptedterms"]
        accepted_terms -> Bool,
        #[sql_name="usertype"]
        user_type -> Nullable<Bool>,
    }
}

joinable!(accesscode -> dailyqueue (queue_id));
joinable!(accesscode -> visitor (visitor_id));

allow_tables_to_appear_in_same_query!(
    accesscode,
    dailyqueue,
    visitor,
);
