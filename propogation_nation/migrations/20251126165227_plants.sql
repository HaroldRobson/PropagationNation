-- Add migration script here
CREATE TABLE plants (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description VARCHAR(5000) NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  location VARCHAR(255) NOT NULL,
  user_id INTEGER REFERENCES users(id),
  picture_urls TEXT[],
  lat REAL NOT NULL,
  lon REAL NOT NULL
);
