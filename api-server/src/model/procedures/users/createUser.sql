CREATE PROCEDURE createUser(
    in emailAddress varchar(255),
    in usersFirstName varchar(255),
    in usersLastName varchar(255),
    in usersUserName varchar(60),
    in passwordHash char(96)
) as $$
BEGIN
    with userId as (
        insert into users.user(email, firstname, lastname)
        values(emailAddress, usersFirstName, usersLastName)
        returning id
    )
    insert into users.passwordAuthentication(id, username, hashedPassword)
    values((select id from userId), usersUserName, passwordHash) returning id;
END
$$