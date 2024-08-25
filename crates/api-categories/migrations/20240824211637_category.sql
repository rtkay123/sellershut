create table category (
    idx serial primary key,
    id varchar(21) unique not null,
    name varchar not null,
    sub_categories varchar(21)[] not null, -- array of ids
    image_url varchar, -- optional image url
    parent_id varchar(21) references category(id) on delete cascade, -- foreign key to self
    created_at timestamptz not null, -- timestamp for creation
    updated_at timestamptz not null -- timestamp for last update
);

create index idx_category_id on category (id);
create index idx_category_name on category (name);
create index idx_category_parent_id on category (parent_id);
create index idx_category_idx_id_parent on category (idx, id, parent_id);
