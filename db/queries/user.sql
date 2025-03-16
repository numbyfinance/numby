--: User()
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
