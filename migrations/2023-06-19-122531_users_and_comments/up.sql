-- Your SQL goes here
create table users (
    id serial primary key,
    ctime timestamp with time zone DEFAULT now(),
    username character varying(255) not null,
    password character varying(255) not null
);
create table comment (
    id serial ,
    user_id serial references users(id),
    ctime timestamp with time zone DEFAULT now(),
    content text,
    primary key(id, user_id)
);