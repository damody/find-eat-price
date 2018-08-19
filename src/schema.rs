table! {
    blacklist (member_id, restaurant_id) {
        member_id -> Integer,
        restaurant_id -> Integer,
        comment -> Varchar,
    }
}

table! {
    chain (chain_id) {
        chain_id -> Integer,
        name -> Varchar,
        good -> Integer,
        bad -> Integer,
        comment -> Varchar,
    }
}

table! {
    chain_tag (chain_id, tag) {
        chain_id -> Integer,
        tag -> Integer,
    }
}

table! {
    like_restaurant (member_id, restaurant_id) {
        member_id -> Integer,
        restaurant_id -> Integer,
        good -> Tinyint,
        comment -> Varchar,
    }
}

table! {
    member (member_id) {
        member_id -> Integer,
        email -> Varchar,
        name -> Varchar,
        enable -> Bit,
        phone_number -> Varchar,
        password -> Varchar,
        member_level -> Tinyint,
        join_date -> Datetime,
    }
}

table! {
    member_vip_record (member_id, member_level) {
        member_id -> Integer,
        member_level -> Tinyint,
        expire_date -> Date,
    }
}

table! {
    menu (menu_id) {
        menu_id -> Integer,
        restaurant_id -> Integer,
        tag -> Integer,
        name -> Varchar,
        price -> Integer,
        input_date -> Datetime,
    }
}

table! {
    menu_tag (menu_id, tag) {
        menu_id -> Integer,
        tag -> Integer,
    }
}

table! {
    restaurant_info (restaurant_id) {
        restaurant_id -> Integer,
        chain_store_id -> Integer,
        name -> Varchar,
        good -> Integer,
        bad -> Integer,
        menu_id -> Integer,
        start_time -> Varchar,
        end_time -> Varchar,
        input_date -> Datetime,
    }
}

table! {
    restaurant_pos (restaurant_id) {
        restaurant_id -> Integer,
        lng -> Float,
        lat -> Float,
        twd97x -> Float,
        twd97y -> Float,
    }
}

table! {
    restaurant_tag (restaurant_id, tag) {
        restaurant_id -> Integer,
        tag -> Integer,
    }
}

table! {
    tag_name (tag) {
        tag -> Integer,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    blacklist,
    chain,
    chain_tag,
    like_restaurant,
    member,
    member_vip_record,
    menu,
    menu_tag,
    restaurant_info,
    restaurant_pos,
    restaurant_tag,
    tag_name,
);
