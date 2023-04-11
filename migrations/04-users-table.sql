CREATE TABLE users (
	user_id uuid NOT NULL DEFAULT gen_random_uuid(),
	first_name text NOT NULL,
	last_name text NOT NULL,
	email_address citext NOT NULL,
	"password" text NOT NULL,
	is_active bool NOT NULL DEFAULT false,
	CONSTRAINT user_email_address_un UNIQUE (email_address),
	CONSTRAINT user_pk PRIMARY KEY (id)
);
