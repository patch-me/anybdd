-- Your SQL goes here
CREATE TABLE users (
  id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  username VARCHAR(255) NOT NULL UNIQUE,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
CREATE TABLE user_gitlab (
  user_id INT NOT NULL,
  gitlab_token VARCHAR(255) NOT NULL, -- Assuming the token is a string
  PRIMARY KEY (user_id),
  FOREIGN KEY (user_id) REFERENCES users(id)
);
CREATE TABLE user_github (
  user_id INT NOT NULL,
  github_token VARCHAR(255) NOT NULL, -- Assuming the token is a string
  PRIMARY KEY (user_id),
  FOREIGN KEY (user_id) REFERENCES users(id)
);
