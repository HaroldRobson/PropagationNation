CREATE TABLE user_details (
  id INTEGER PRIMARY KEY REFERENCES users,
  photo_url TEXT,
  location VARCHAR(255) NOT NULL -- rough string of where they live  
);-- Add migration script here
