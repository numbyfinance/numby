-- Add up migration script here

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
    email text unique not null,
    name text not null,
    password text not null,
    created_at timestamp with time zone default current_timestamp,
    updated_at timestamp with time zone default current_timestamp,
    tombstone boolean default false
);

create index idx_users_id on users(id);

create table if not exists groups (
    name text not null unique primary key
);

create table if not exists permissions (
    name text not null unique primary key
);

create table if not exists users_groups (
    user_id uuid references users(id),
    "group" text references groups(name),
    primary key (user_id, "group")
);

create table if not exists groups_permissions (
    "group" text references groups(name),
    permission text references permissions(name),
    primary key ("group", permission)
);

insert into users (id, email, name, password)
values (
    '0195989c-d1db-7a70-9060-9cea4a6b8332',
    'topaz@ipc.org',
    'Topaz & Numby',
    '$argon2id$v=19$m=19456,t=2,p=1$hmH0Kladr68gSnEwAFV9xQ$qmqH96rVX7OTJRsxjfInwboRZ9fh77t/63brhO0Usz0'
);

insert into groups (name) values ('users');
insert into permissions (name) values ('protected.read');

insert into groups_permissions ("group", permission)
values ('users', 'protected.read');

insert into users_groups (user_id, "group")
values ((select id from users where email = 'topaz@ipc.org'), 'users');
