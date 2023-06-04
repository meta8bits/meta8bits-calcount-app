create extension if not exists "uuid-ossp";

create table users(
    id serial primary key,
    username varchar(255) unique not null,
    email varchar(255) unique not