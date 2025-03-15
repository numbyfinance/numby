-- while we wait for postgresql 18
-- https://gist.github.com/kjmph/5bd772b2c2df145aa645b837da7eca74
create or replace function uuid_generate_v7()
returns uuid
as $$
begin
  -- use random v4 uuid as starting point (which has the same variant we need)
  -- then overlay timestamp
  -- then set version 7 by flipping the 2 and 1 bit in the version 4 string
  return encode(
    set_bit(
      set_bit(
        overlay(uuid_send(gen_random_uuid())
                placing substring(int8send(floor(extract(epoch from clock_timestamp()) * 1000)::bigint) from 3)
                from 1 for 6
        ),
        52, 1
      ),
      53, 1
    ),
    'hex')::uuid;
end
$$
language plpgsql
volatile;

create table if not exists users (
    id uuid default public.uuid_generate_v7() primary key,
    email varchar(255) unique not null,
    name varchar(255) not null,
    password varchar(255) not null,
    created_at timestamp with time zone default current_timestamp,
    updated_at timestamp with time zone default current_timestamp,
    tombstone boolean default false
);

create index idx_users_email on users(email);

create table if not exists groups (
    id uuid default public.uuid_generate_v7() primary key,
    name text not null unique
);

create table if not exists permissions (
    id uuid default public.uuid_generate_v7() primary key,
    name text not null unique
);

create table if not exists users_groups (
    user_id uuid references users(id),
    group_id uuid references groups(id),
    primary key (user_id, group_id)
);

create table if not exists groups_permissions (
    group_id uuid references groups(id),
    permission_id uuid references permissions(id),
    primary key (group_id, permission_id)
);

insert into users (id, email, name, password)
values (
    '0195989c-d1db-7a70-9060-9cea4a6b8332',
    'topaz@ipc.org',
    'Topaz & Numby',
    '$argon2id$v=19$m=19456,t=2,p=1$hmH0Kladr68gSnEwAFV9xQ$qmqH96rVX7OTJRsxjfInwboRZ9fh77t/63brhO0Usz0'
);

insert into permissions (name) values ('protected.read');
insert into permissions (name) values ('restricted.read');

insert into groups (name) values ('users');
insert into groups (name) values ('superusers');

insert into groups_permissions (group_id, permission_id)
values (
    (select id from groups where name = 'users'),
    (select id from permissions where name = 'protected.read')
), (
    (select id from groups where name = 'superusers'),
    (select id from permissions where name = 'restricted.read')
);

insert into users_groups (user_id, group_id)
values (
    (select id from users where email = 'topaz@ipc.org'),
    (select id from groups where name = 'users')
), (
    (select id from users where email = 'topaz@ipc.org'),
    (select id from groups where name = 'superusers')
);
