--: User()
--! get_user_by_email : User
SELECT
    *
FROM
    users
WHERE
    email = :email;

--! get_user_by_id : User
SELECT
    *
FROM
    users
WHERE
    id = :id;

--! get_permissions
--- Gets all permissions from all groups a user is in.
SELECT
    DISTINCT permissions.name
FROM
    users
    JOIN users_groups ON users.id = users_groups.user_id
    JOIN groups_permissions ON users_groups.group_id = groups_permissions.group_id
    JOIN permissions ON groups_permissions.permission_id = permissions.id
WHERE
    users.id = :id;