CREATE TABLE participations (
  id SERIAL PRIMARY KEY,

  event_id INTEGER REFERENCES events(id) NOT NULL,
  user_id INTEGER REFERENCES users(id) NOT NULL
);
