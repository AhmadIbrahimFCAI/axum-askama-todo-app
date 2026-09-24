-- Add migration script here
create extension if not exists citext;

create table if not exists users (
    id bigserial not null primary key,
    email citext unique not null,
    password_hash bytea not null
);
