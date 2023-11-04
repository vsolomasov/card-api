--liquibase formatted sql
--changeset v.solomasov:create-tables

create table if not exists identity (
    id uuid primary key,
    login varchar not null unique,
    email varchar not null unique,
    salt uuid not null,
    password varchar not null
);