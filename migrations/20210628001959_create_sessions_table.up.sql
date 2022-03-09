CREATE TABLE sessions (
	id uuid DEFAULT gen_random_uuid() UNIQUE,
	name varchar(512),
	user_id uuid NOT NULL,
	expired_after timestamp DEFAULT now_utc() + ('1 month')::interval NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX ON sessions(expired_after);

CREATE OR REPLACE FUNCTION delete_expired_sessions() RETURNS trigger AS $$
	BEGIN
		DELETE FROM sessions WHERE now_utc() > expired_after;
	  RETURN NULL;
	END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_delete_expired_sessions
	AFTER INSERT ON sessions
	EXECUTE PROCEDURE delete_expired_sessions();

CREATE VIEW active_sessions AS
	SELECT * FROM sessions WHERE now_utc() <= expired_after;
