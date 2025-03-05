-- while we wait for postgresql 18
-- https://gist.github.com/kjmph/5bd772b2c2df145aa645b837da7eca74
CREATE
OR REPLACE FUNCTION uuid_generate_v7() RETURNS uuid AS
$$
BEGIN
-- use random v4 uuid as starting point (which has the same variant we need)
-- then overlay timestamp
-- then set version 7 by flipping the 2 and 1 bit in the version 4 string
RETURN encode(
    set_bit(
        set_bit(
            overlay(
                uuid_send(gen_random_uuid()) placing substring(
                    int8send(
                        floor(
                            extract(
                                epoch
                                FROM
                                    clock_timestamp()
                            ) * 1000
                        ) :: bigint
                    )
                    FROM
                        3
                )
                FROM
                    1 FOR 6
            ),
            52,
            1
        ),
        53,
        1
    ),
    'hex'
) :: uuid;

END
$$
language plpgsql volatile;

CREATE TABLE users (
    id uuid DEFAULT public.uuid_generate_v7() PRIMARY KEY,
    email varchar(255) UNIQUE NOT NULL,
    name varchar(255) NOT NULL,
    password_hash varchar(255) NOT NULL,
    created_at timestamp WITH time zone DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp WITH time zone DEFAULT CURRENT_TIMESTAMP,
    is_active boolean DEFAULT TRUE
);

CREATE INDEX idx_users_email ON users(email);
