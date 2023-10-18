CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE uuid_users (
  user_uid uuid DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL,
  PRIMARY KEY (user_uid)
)

