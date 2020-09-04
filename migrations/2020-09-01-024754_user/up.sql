-- Your SQL goes here
-- 系统用户表
CREATE TABLE sys_user (
    user_id bigint NOT NULL PRIMARY KEY,
    --'主键id'
    avatar varchar(255),
    --'头像'
    account varchar(45),
    --'账号'
    password varchar(45),
    --'密码'
    salt varchar(45),
    --'md5密码盐'
    name varchar(45),
    --'名字'
    birthday datetime,
    --'生日'
    sex varchar(32),
    --'性别(字典)'
    email varchar(45),
    --'电子邮件'
    phone varchar(45),
    --'电话'
    status varchar(32),
    --'状态(字典)'
    create_time datetime,
    --'创建时间'
    create_user bigint,
    --'创建人'
    update_time datetime,
    --'更新时间'
    update_user bigint --'更新人'
);