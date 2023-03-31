create table inventory
(
    id         bigserial not null,
    name       text      not null,
    price      money     not null,
    constraint inventory_pk primary key (id)
);

