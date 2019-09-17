create procedure login(
  in userId bigint,
  in tokenHash char(96)
) as $$
begin
    insert into users.tokenAuthentication(userId, hashedToken)
     values (userId, tokenHash)
      returning id;
end
$$;