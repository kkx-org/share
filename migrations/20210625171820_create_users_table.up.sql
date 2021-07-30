CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE OR REPLACE FUNCTION now_utc() RETURNS timestamp AS $$
  SELECT now() at time zone 'utc';
$$ LANGUAGE sql;

CREATE TABLE users (
	id uuid DEFAULT gen_random_uuid() UNIQUE,
	username varchar(64) NOT NULL UNIQUE,
	email varchar(64) NOT NULL UNIQUE,
	hashed_password varchar(512) NOT NULL,
	created_at timestamp DEFAULT now_utc(),
	PRIMARY KEY (id)
);

CREATE INDEX ON users(username);
CREATE INDEX ON users(email);
