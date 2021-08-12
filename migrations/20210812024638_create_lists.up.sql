-- Create "lists" table

CREATE TABLE lists (
  id BIGSERIAL PRIMARY KEY,
  board_id BIGINT NOT NULL,
  title TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

ALTER TABLE lists
  ADD CONSTRAINT fk_board_id
  FOREIGN KEY (board_id)
  REFERENCES boards(id)
  ON DELETE CASCADE;
