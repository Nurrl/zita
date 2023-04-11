CREATE TABLE applications (
	application_id uuid NOT NULL DEFAULT gen_random_uuid(),
	"name" text NOT NULL,
	description text NULL,
	CONSTRAINT application_pk PRIMARY KEY (application_id)
);
