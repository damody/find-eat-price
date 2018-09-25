table! {
    blacklist (member_email, restaurant_id) {
        member_email -> Varchar,
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
        pic_urls -> Varchar,
    }
}

table! {
    chain_tag (chain_id, tag_id) {
        chain_id -> Integer,
        tag_id -> Integer,
        pic_url -> Varchar,
    }
}

table! {
    food (food_id) {
        food_id -> Integer,
        menu_id -> Integer,
        food_name -> Varchar,
        price -> Float,
        pic_urls -> Varchar,
        input_date -> Datetime,
    }
}

table! {
    food_log (modify_date) {
        food_id -> Integer,
        member_email -> Varchar,
        discrption -> Varchar,
        modify_date -> Datetime,
    }
}

table! {
    food_tag (food_id, tag_id) {
        food_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    like_restaurant (member_email, restaurant_id) {
        member_email -> Varchar,
        restaurant_id -> Integer,
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
        menu_id -> Integer,
        pic_urls -> Varchar,
        input_date -> Datetime,
    }
}

table! {
    menu_tag (menu_id, tag_id) {
        menu_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    restaurant (restaurant_id) {
        restaurant_id -> Integer,
        author_email -> Varchar,
        chain_id -> Integer,
        menu_id -> Integer,
        name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        enable -> Tinyint,
        good -> Integer,
        bad -> Integer,
        open_time -> Varchar,
        close_time -> Varchar,
        delivery -> Varchar,
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
        restaurant_id -> Integer,
        member_email -> Varchar,
        discrption -> Varchar,
        modify_date -> Datetime,
    }
}

table! {
    restaurant_tag (restaurant_id, tag_id) {
        restaurant_id -> Integer,
        tag_id -> Integer,
        pic_url -> Varchar,
    }
}

table! {
    tag_name (tag_id) {
        tag_id -> Integer,
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
