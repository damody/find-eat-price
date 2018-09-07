table! {
    blacklist (member_email, restaurant_id) {
        member_email -> Varchar,
        restaurant_id -> Varchar,
        comment -> Varchar,
    }
}

table! {
    chain (chain_id) {
        chain_id -> Varchar,
        name -> Varchar,
        good -> Integer,
        bad -> Integer,
        comment -> Varchar,
        pic_urls -> Varchar,
    }
}

table! {
    chain_tag (chain_id, tname) {
        chain_id -> Varchar,
        tname -> Varchar,
        pic_url -> Varchar,
    }
}

table! {
    food (food_id) {
        food_id -> Varchar,
        menu_id -> Varchar,
        name -> Varchar,
        price -> Float,
        pic_urls -> Varchar,
        input_date -> Datetime,
    }
}

table! {
    food_log (modify_date) {
        food_id -> Varchar,
        member_email -> Varchar,
        discrption -> Varchar,
        modify_date -> Datetime,
    }
}

table! {
    food_tag (food_id, tname) {
        food_id -> Varchar,
        tname -> Varchar,
    }
}

table! {
    like_restaurant (member_email, restaurant_id) {
        member_email -> Varchar,
        restaurant_id -> Varchar,
        good -> Tinyint,
        comment -> Varchar,
    }
}

table! {
    member (member_email) {
        member_email -> Varchar,
        name -> Varchar,
        enable -> Tinyint,
        gender -> Tinyint,
        phone -> Varchar,
        password -> Varchar,
        member_level -> Tinyint,
        join_date -> Datetime,
    }
}

table! {
    member_log (modify_date) {
        member_email -> Varchar,
        discrption -> Varchar,
        modify_date -> Datetime,
    }
}

table! {
    member_vip_record (member_email, member_level) {
        member_email -> Varchar,
        member_level -> Tinyint,
        start_date -> Date,
        expire_date -> Date,
    }
}

table! {
    menu (menu_id) {
        menu_id -> Varchar,
        pic_urls -> Varchar,
        input_date -> Datetime,
    }
}

table! {
    menu_tag (menu_id, tname) {
        menu_id -> Varchar,
        tname -> Varchar,
    }
}

table! {
    restaurant (restaurant_id) {
        restaurant_id -> Varchar,
        author_email -> Varchar,
        chain_id -> Varchar,
        menu_id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        enable -> Tinyint,
        good -> Integer,
        bad -> Integer,
        open_time -> Varchar,
        close_time -> Varchar,
        lng -> Float,
        lat -> Float,
        twd97x -> Float,
        twd97y -> Float,
        pic_urls -> Varchar,
        input_date -> Datetime,
    }
}

table! {
    restaurant_log (modify_date) {
        restaurant_id -> Varchar,
        member_email -> Varchar,
        discrption -> Varchar,
        modify_date -> Datetime,
    }
}

table! {
    restaurant_tag (restaurant_id, tname) {
        restaurant_id -> Varchar,
        tname -> Varchar,
        pic_url -> Varchar,
    }
}

table! {
    tag_name (tname) {
        tname -> Varchar,
        pic_url -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    blacklist,
    chain,
    chain_tag,
    food,
    food_log,
    food_tag,
    like_restaurant,
    member,
    member_log,
    member_vip_record,
    menu,
    menu_tag,
    restaurant,
    restaurant_log,
    restaurant_tag,
    tag_name,
);
