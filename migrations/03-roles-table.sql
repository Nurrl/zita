CREATE TABLE roles (
	role_id uuid NOT NULL DEFAULT gen_random_uuid(),
	"name" text NOT NULL,
	description text NULL,
	CONSTRAINT policies_pk PRIMARY KEY (role_id)
);
