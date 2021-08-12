-- Undo commands executed in `up.sql` file

DROP TABLE tickets;

DROP FUNCTION IF EXISTS trigger_set_timestamp;
DROP TRIGGER IF EXISTS set_timestamp ON tickets;

