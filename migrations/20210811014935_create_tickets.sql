-- Define PostgreSQL function to set updated_at field to `now()`
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create the "tickets" table
CREATE TABLE tickets (
    id BIGSERIAL PRIMARY KEY,
    list_id BIGINT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Create trigger for "updated_at" field
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON tickets
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Add references
ALTER TABLE tickets
  ADD CONSTRAINT fk_list_id
  FOREIGN KEY (list_id)
  REFERENCES lists(id)
  ON DELETE CASCADE;
