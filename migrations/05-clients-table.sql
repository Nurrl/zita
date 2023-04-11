CREATE TABLE zita.clients (
	client_id uuid NOT NULL DEFAULT gen_random_uuid(),
	client_secret text NOT NULL,
	application_id uuid NOT NULL,
	CONSTRAINT clients_pk PRIMARY KEY (client_id),
	CONSTRAINT clients_applications_fk FOREIGN KEY (application_id) REFERENCES zita.applications(application_id)
);
