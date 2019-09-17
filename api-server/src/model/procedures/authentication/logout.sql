create procedure logout(
  in tokenId bigint,
  in tokenHash char(96)
) as $$
declare token char(96);
begin
    select hashedToken
    into token
    from users.tokenAuthentication
    where id = tokenId;
    if (token = tokenHash) then
      delete from users.tokenAuthentication where id = tokenId;
    end if;
end
$$;