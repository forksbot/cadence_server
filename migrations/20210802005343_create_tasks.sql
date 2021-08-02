-- create_tasks.sql
CREATE TYPE STATUS AS ENUM ('pending', 'doing', 'complete');

CREATE TABLE IF NOT EXISTS tasks (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  workspace_id uuid NOT NULL,
  description TEXT NOT NULL,
  status STATUS NOT NULL DEFAULT 'doing',
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (current_timestamp AT TIME ZONE 'utc'),
  CONSTRAINT workspace_fk
    FOREIGN KEY (workspace_id)
    REFERENCES workspaces(id)
    ON DELETE CASCADE
);

-- seed database with test data for development
INSERT INTO tasks (workspace_id, description, status)
VALUES
('ebd17df8-a523-433a-ada4-26ebe88f8f69', 'test card 1', 'pending'),
('ebd17df8-a523-433a-ada4-26ebe88f8f69', 'test card 2', 'doing'),
('ebd17df8-a523-433a-ada4-26ebe88f8f69', 'test card 3', 'complete'),
('520410dc-ef63-430c-bdbb-764e0e520bcb', 'test card 4', 'pending'),
('520410dc-ef63-430c-bdbb-764e0e520bcb', 'test card 5', 'doing'),
('daa13224-7880-46f7-a152-9b7df32aa5ec', 'test card 6', 'doing'),
('daa13224-7880-46f7-a152-9b7df32aa5ec', 'test card 7', 'complete');
