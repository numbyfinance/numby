--: User()
--: ApiToken(expires_at?)

--! get_user_by_email : User
select * from users where email = :email;

--! get_user_by_id : User
select * from users where id = :id;

--! get_permissions
--- Gets all permissions from all groups a user is in.
select distinct permissions.name
from users
    join users_groups on users.id = users_groups.user_id
    join groups_permissions on users_groups.group = groups_permissions.group
    join permissions on groups_permissions.permission = permissions.name
where
    users.id = :id;

--! get_api_token : ApiToken
select * from api_tokens
where token = :token
    and (expires_at is null or expires_at > now());

--! get_user_from_api_token : User
select users.*
from api_tokens
    join users on api_tokens.user_id = users.id
where
    token = :token
    and (expires_at is null or expires_at > now());
