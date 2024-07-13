create table user (
    id varchar(21) primary key,
    username varchar(25) not null,
    email varchar not null,
    name varchar(255), -- optional name
    avatar varchar, -- optional avatar
    group_id varchar(21), -- foreign key to groups
    created_at bigint not null, -- millisecond precision timestamp for creation
    updated_at bigint not null -- millisecond precision timestamp for last update
);

create table group (
    id varchar(21) primary key,
    name varchar(255), -- optional name
    description varchar(255), -- optional avatar
    admin_id varchar(21) references user(id),
    created_at bigint not null, -- millisecond precision timestamp for creation
    updated_at bigint not null -- millisecond precision timestamp for last update
);

alter table user add constraint fk_group_id foreign key (group_id) references group(id);
alter table group add constraint fk_admin_id foreign key (admin_id) references user(id);

create index idx_groups_admin_id on group(admin_id);
create index idx_groups_name on group(name);

create index idx_users_username on user(username);
create index idx_users_email on user(email);
create index idx_users_group_id on user(group_id);

create index idx_users_username_email on users(username, email);
