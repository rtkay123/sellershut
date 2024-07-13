create table oauth_account (
    provider_id varchar(21),
    provider_user_id varchar(50),
    user_id varchar(21),
    constraint pk_users primary key (provider_id, provider_user_id)
);
