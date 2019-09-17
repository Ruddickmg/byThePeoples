create procedure authenticateToken(
  in name bigint,
  in passwordHash char(96)
) as $$
declare
    id bigint := null;
begin
    select userId
     into id
      from users.passwordAuthentication
       where userName = name
         and hashedPassword = passwordHash;
    select id;
end
$$;