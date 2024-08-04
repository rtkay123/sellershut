create table oauth_account (
    provider_id text not null,
    provider_user_id text not null,
    user_id varchar(21) not null,
    primary key (provider_id, provider_user_id),
    foreign key (user_id) references "user"(id) on delete cascade
);

create table oauth_session (
    id varchar(255) primary key,
    user_id varchar(21) not null,
    expires_at bigint not null,
    foreign key (user_id) references "user"(id) on delete cascade
);
