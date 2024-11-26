-- Your SQL goes here
CREATE TABLE job_status (
                            id UUID PRIMARY KEY,
                            customer_id VARCHAR NOT NULL,
                            cairo_job_key VARCHAR NOT NULL,
                            status VARCHAR NOT NULL,
                            validation_done BOOLEAN NOT NULL,
                            created_on TIMESTAMP NOT NULL DEFAULT NOW()
);