-- Add migration script here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255) UNIQUE NOT NULL,
  password VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  description VARCHAR(5000) NOT NULL,
  lat REAL,
  lon REAL,
  created_at TIMESTAMP DEFAULT NOW()
);
