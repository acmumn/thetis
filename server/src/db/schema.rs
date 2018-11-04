#![allow(proc_macro_derive_resolution_fallback)]

table! {
    jwt_escrow (id) {
        id -> Unsigned<Integer>,
        member_id -> Unsigned<Integer>,
        uuid -> Binary,
        created -> Datetime,
    }
}

table! {
    mailing_lists (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        deleted -> Bool,
        hidden -> Bool,
    }
}

table! {
    mail_global_unsubscribes (id) {
        id -> Unsigned<Integer>,
        email -> Varchar,
    }
}

table! {
    mail_list_unsubscribes (id) {
        id -> Unsigned<Integer>,
        email -> Varchar,
        mailing_list_id -> Unsigned<Integer>,
    }
}

table! {
    mail_member_subscriptions (id) {
        id -> Unsigned<Integer>,
        member_id -> Unsigned<Integer>,
        mailing_list_id -> Unsigned<Integer>,
    }
}

table! {
    mail_other_subscriptions (id) {
        id -> Unsigned<Integer>,
        email -> Varchar,
        mailing_list_id -> Unsigned<Integer>,
    }
}

table! {
    mail_send_queue (id) {
        id -> Unsigned<Integer>,
        template_id -> Unsigned<Integer>,
        data -> Longtext,
        email -> Varchar,
        subject -> Varchar,
        send_after -> Datetime,
        send_started -> Bool,
        send_done -> Bool,
    }
}

table! {
    mail_templates (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        contents -> Longtext,
        markdown -> Bool,
    }
}

table! {
    members (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        x500 -> Varchar,
        card -> Nullable<Char>,
        email -> Varchar,
    }
}

table! {
    members_tag_join (id) {
        id -> Unsigned<Integer>,
        member_id -> Unsigned<Integer>,
        tags_id -> Unsigned<Integer>,
    }
}

table! {
    member_bans (id) {
        id -> Unsigned<Integer>,
        member_id -> Unsigned<Integer>,
        issued_by -> Nullable<Unsigned<Integer>>,
        date_from -> Datetime,
        date_to -> Nullable<Datetime>,
        invalidated_at -> Nullable<Datetime>,
        invalidated_by -> Nullable<Unsigned<Integer>>,
        notes -> Text,
    }
}

table! {
    member_payments (id) {
        id -> Unsigned<Integer>,
        member_id -> Unsigned<Integer>,
        date_from -> Datetime,
        date_to -> Datetime,
        notes -> Text,
    }
}

table! {
    tags (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
    }
}

joinable!(jwt_escrow -> members (member_id));
joinable!(mail_list_unsubscribes -> mailing_lists (mailing_list_id));
joinable!(mail_member_subscriptions -> mailing_lists (mailing_list_id));
joinable!(mail_member_subscriptions -> members (member_id));
joinable!(mail_other_subscriptions -> mailing_lists (mailing_list_id));
joinable!(mail_send_queue -> mail_templates (template_id));
joinable!(member_payments -> members (member_id));
joinable!(members_tag_join -> members (member_id));
joinable!(members_tag_join -> tags (tags_id));

allow_tables_to_appear_in_same_query!(
    jwt_escrow,
    mailing_lists,
    mail_global_unsubscribes,
    mail_list_unsubscribes,
    mail_member_subscriptions,
    mail_other_subscriptions,
    mail_send_queue,
    mail_templates,
    members,
    members_tag_join,
    member_bans,
    member_payments,
    tags,
);
