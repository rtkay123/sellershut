create table "group" (
    id varchar(21) primary key,
    name varchar(255) not null, -- optional name
    description varchar(255), -- optional avatar
    created_at bigint not null, -- millisecond precision timestamp for creation
    updated_at bigint not null -- millisecond precision timestamp for last update
);

create table "user" (
    id varchar(21) primary key,
    username varchar(25) unique not null,
    email varchar(50) unique not null,
    name varchar(255), -- optional name
    avatar varchar, -- optional avatar
    group_id varchar(21), -- foreign key to groups
    created_at bigint not null, -- millisecond precision timestamp for creation
    updated_at bigint not null, -- millisecond precision timestamp for last update
    foreign key (group_id) references "group"(id)
);

create table role (
    id serial primary key,
    name varchar(50) unique not null, -- optional name
    description varchar(255), -- optional avatar
    created_at bigint not null, -- millisecond precision timestamp for creation
    updated_at bigint not null -- millisecond precision timestamp for last update
);

create table permission (
    id serial primary key,
    name varchar(50) unique not null, -- optional name
    description varchar(255), -- optional avatar
    created_at bigint not null, -- millisecond precision timestamp for creation
    updated_at bigint not null -- millisecond precision timestamp for last update
);

create table role_permission (
    role_id integer not null,
    permission_id integer not null,
    primary key (role_id, permission_id),
    foreign key (role_id) references role(id) on delete cascade,
    foreign key (permission_id) references permission(id) on delete cascade
);

create table user_role (
    user_id varchar(50) not null,
    role_id integer not null,
    group_id varchar(21) null,
    primary key (user_id, role_id, group_id),
    foreign key (user_id) references "user"(id) on delete cascade,
    foreign key (role_id) references role(id) on delete cascade,
    foreign key (group_id) references "group"(id) on delete cascade
);

create index idx_group_name on "group"(name);

create index idx_user_username on "user"(username);
create index idx_user_email on "user"(email);
create index idx_user_group_id on "user"(group_id);

create index idx_user_username_email on "user"(username, email);
