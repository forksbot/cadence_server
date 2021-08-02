-- create_workspaces.sql
CREATE TABLE IF NOT EXISTS workspaces (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(50) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (current_timestamp AT TIME ZONE 'utc')
);

-- seed database
INSERT INTO workspaces (name) VALUES ('test board 1'), ('test board 2'), ('test board 3');
