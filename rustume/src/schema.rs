// @generated automatically by Diesel CLI.

diesel::table! {
    client (client_id) {
        client_id -> Int4,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 20]
        client_password -> Varchar,
        #[max_length = 50]
        email -> Varchar,
        admin_privilege -> Bool,
    }
}

diesel::table! {
    client_to_room (client_room_id) {
        client_room_id -> Int4,
        client_id -> Int4,
        room_id -> Int4,
        delete_privilege -> Bool,
        edit_privilege -> Bool,
        write_privilege -> Bool,
    }
}

diesel::table! {
    room (room_id) {
        room_id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 500]
        elucidation -> Varchar,
    }
}

diesel::table! {
    room_subject (subject_id) {
        subject_id -> Int4,
        client_room_id -> Int4,
        room_id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 500]
        elucidation -> Varchar,
        #[max_length = 50]
        subject_type -> Varchar,
    }
}

diesel::table! {
    subject_chat (chat_id) {
        chat_id -> Int4,
        subject_id -> Int4,
        client_id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 500]
        elucidation -> Varchar,
        user_message -> Varchar,
    }
}

diesel::table! {
    user_to_user (user_to_user_id) {
        user_to_user_id -> Int4,
        user_id -> Nullable<Int4>,
        users_chat_id -> Nullable<Int4>,
    }
}

diesel::table! {
    user_to_user_message (user_to_user_message_id) {
        user_to_user_message_id -> Int4,
        user_to_user_chat_id -> Int4,
        client_id -> Int4,
        user_message -> Varchar,
    }
}

diesel::table! {
    user_to_user_room (user_to_user_chat_id) {
        user_to_user_chat_id -> Int4,
        subject_id -> Int4,
        client_id -> Int4,
    }
}

diesel::joinable!(client_to_room -> client (client_id));
diesel::joinable!(client_to_room -> room (room_id));
diesel::joinable!(room_subject -> client_to_room (client_room_id));
diesel::joinable!(room_subject -> room (room_id));
diesel::joinable!(subject_chat -> client (client_id));
diesel::joinable!(subject_chat -> room_subject (subject_id));
diesel::joinable!(user_to_user -> client (user_id));
diesel::joinable!(user_to_user -> subject_chat (users_chat_id));
diesel::joinable!(user_to_user_message -> client (client_id));
diesel::joinable!(user_to_user_message -> user_to_user_room (user_to_user_chat_id));
diesel::joinable!(user_to_user_room -> client (client_id));
diesel::joinable!(user_to_user_room -> user_to_user (subject_id));

diesel::allow_tables_to_appear_in_same_query!(
    client,
    client_to_room,
    room,
    room_subject,
    subject_chat,
    user_to_user,
    user_to_user_message,
    user_to_user_room,
);
