create table category (
    id varchar(21) primary key,
    name varchar not null,
    sub_categories varchar(21)[], -- array of ids
    image_url varchar, -- optional image url
    parent_id varchar(21) references category(id) on delete cascade, -- foreign key to self
    created_at bigint not null, -- millisecond precision timestamp for creation
    updated_at bigint not null -- millisecond precision timestamp for last update
);

create index idx_category_name on category (name);
create index idx_category_parent_id on category (parent_id);
create index idx_category_created_at_id_parent on category (created_at, id, parent_id);
