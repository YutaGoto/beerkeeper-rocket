CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  location VARCHAR,
  max_size INTEGER NOT NULL,
  budget VARCHAR,
  description TEXT,
  start_at VARCHAR,
  end_at VARCHAR,
  organizer_id INTEGER REFERENCES users(id) NOT NULL
)
