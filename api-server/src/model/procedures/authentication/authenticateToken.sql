create procedure authenticateToken(
  in tokenId bigint,
  in tokenHash char(96)
) as $$
declare
    today timestamp := now();
    lastLogin timestamp := null;
    dummyId bigint := 1;
    usersId bigint := null;
begin
    select userId, updatedAt
     into usersId, lastLogin
      from users.tokenAuthentication
       where id = tokenId and hashedToken = tokenHash;
    if (TRUNC(DATE_PART('day', today - lastLogin)) > 7 or usersId is null) then
        update users.tokenAuthentication
        set updatedAt = today
        where id = dummyId;
    else
        update users.tokenAuthentication
        set updatedAt = today
        where id = tokenId;
    end if;
    select usersId as id;
end
$$;