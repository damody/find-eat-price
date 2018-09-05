-- 會員資訊
-- name 名字
-- password 密碼的hash
-- join_date 加入日期
-- member_level 會員等級
CREATE TABLE member
(
    member_id INT NOT NULL AUTO_INCREMENT UNIQUE,
    email VARCHAR(128) NOT NULL UNIQUE,
    name VARCHAR(128) NOT NULL,
    enable TINYINT DEFAULT 1 NOT NULL,
    gender TINYINT DEFAULT 0 NOT NULL,
    phone VARCHAR(32) DEFAULT "" NOT NULL,
    password VARCHAR(64) NOT NULL,
    member_level TINYINT DEFAULT 0 NOT NULL,
    join_date DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY Key(member_id)
);
-- VIP記錄

CREATE TABLE member_vip_record
(
    member_id INT NOT NULL,
    member_level TINYINT NOT NULL,
    expire_date DATE NOT NULL,
    PRIMARY Key(member_id, member_level)
);

-- 喜歡的店家 
-- good 喜歡程度 1~5
-- comment 喜歡的原因
CREATE TABLE like_restaurant
(
    member_id INT NOT NULL,
    restaurant_id INT NOT NULL,
    good TINYINT NOT NULL,
    comment VARCHAR(500) NOT NULL,
    PRIMARY Key(member_id, restaurant_id)
);

CREATE TABLE blacklist
(
    member_id INT NOT NULL,
    restaurant_id INT NOT NULL,
    comment VARCHAR(500) NOT NULL,
    PRIMARY Key(member_id, restaurant_id)
);

-- 店家資訊
-- good 是喜歡數量
-- bad  是不喜歡數量
-- open_time 開店時間
-- close_time 關店時間
-- chain_store_id 連鎖店id

CREATE TABLE restaurant
(
    restaurant_id INT NOT NULL AUTO_INCREMENT UNIQUE,
    author_id INT NOT NULL,
    chain_id INT DEFAULT -1 NOT NULL,
    name VARCHAR(128) NOT NULL,
    email VARCHAR(128) DEFAULT "" NOT NULL,
    phone VARCHAR(32) DEFAULT "" NOT NULL,
    enable TINYINT DEFAULT 1 NOT NULL,
    good INT DEFAULT 0 NOT NULL,
    bad INT DEFAULT 0 NOT NULL,
    menu_id INT DEFAULT -1 NOT NULL,
    open_time VARCHAR(128) DEFAULT '' NOT NULL,
    close_time VARCHAR(128) DEFAULT '' NOT NULL,
    lng FLOAT NOT NULL,
    lat FLOAT NOT NULL,
    twd97x FLOAT NOT NULL,
    twd97y FLOAT NOT NULL,
    input_date DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY Key(restaurant_id)
);

-- 連鎖店
-- good 是喜歡數量
-- bad  是不喜歡數量
CREATE TABLE chain
(
    chain_id INT NOT NULL AUTO_INCREMENT UNIQUE,
    name VARCHAR(128) NOT NULL UNIQUE,
    good INT DEFAULT 0 NOT NULL,
    bad INT DEFAULT 0 NOT NULL,
    comment VARCHAR(500) DEFAULT '' NOT NULL,
    PRIMARY Key(chain_id)
);
-- 標籤：素食、牛肉、雞肉、泰國、川菜等等
CREATE TABLE tag_name
(
    tag INT NOT NULL,
    name VARCHAR(128) NOT NULL,
    PRIMARY Key(tag)
);
-- 連鎖店標籤
CREATE TABLE chain_tag
(
    chain_id INT NOT NULL,
    tag INT NOT NULL,
    PRIMARY Key(chain_id, tag)
);
-- 店面標籤
CREATE TABLE restaurant_tag
(
    restaurant_id INT NOT NULL,
    tag INT NOT NULL,
    PRIMARY Key(restaurant_id, tag)
);

-- 菜單資訊
-- 免費的東西 price 會是0
CREATE TABLE menu
(
    menu_id INT NOT NULL AUTO_INCREMENT UNIQUE,
    input_date DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY Key(menu_id)
);

CREATE TABLE menu_tag
(
    menu_id INT NOT NULL,
    tag INT NOT NULL,
    PRIMARY Key(menu_id, tag)
);

CREATE TABLE food_tag
(
    food_id INT NOT NULL,
    tag INT NOT NULL,
    PRIMARY Key(food_id, tag)
);

CREATE TABLE food
(
    food_id INT NOT NULL AUTO_INCREMENT UNIQUE,
    restaurant_id INT NOT NULL,
    tag INT NOT NULL,
    name VARCHAR(128) NOT NULL,
    price INT NOT NULL,
    input_date DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY Key(food_id)
);

CREATE TABLE food_log
(
    food_id INT NOT NULL,
    member_id INT NOT NULL,
    discrption VARCHAR(128) NOT NULL,
    modify_date DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY Key(modify_date)
);

CREATE TABLE restaurant_log
(
    restaurant_id INT NOT NULL,
    member_id INT NOT NULL,
    discrption VARCHAR(128) NOT NULL,
    modify_date DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY Key(modify_date)
);

CREATE TABLE member_log
(
    member_id INT NOT NULL,
    discrption VARCHAR(128) NOT NULL,
    modify_date DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY Key(modify_date)
);
