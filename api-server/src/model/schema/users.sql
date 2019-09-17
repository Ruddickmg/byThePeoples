create schema users;

alter schema users owner to "GrantRuddick";

create table users.authentication
(
	id bigint not null
		constraint authentication_pk
			primary key
);

alter table users.passwordAuthentication owner to "GrantRuddick";