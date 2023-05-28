-- Your SQL goes here
CREATE TABLE client (
  client_id SERIAL PRIMARY KEY,
  first_name VARCHAR(50) NOT NULL,
  last_name VARCHAR(50) NOT NULL,
  client_password VARCHAR(20) UNIQUE NOT NULL,
  email VARCHAR(50) UNIQUE NOT NULL,
  admin_privilege BOOLEAN NOT NULL
);

CREATE TABLE room (
  room_id SERIAL PRIMARY KEY,
  title VARCHAR(50) UNIQUE NOT NULL,
  elucidation VARCHAR(500) NOT NULL
);

CREATE TABLE client_to_room(
  client_room_id SERIAL PRIMARY KEY,
  client_id INT NOT NULL,
  room_id INT NOT NULL,
  delete_privilege BOOLEAN NOT NULL,
  edit_privilege BOOLEAN NOT NULL,
  write_privilege BOOLEAN NOT NULL,
  FOREIGN KEY (client_id) REFERENCES client(client_id),
  FOREIGN KEY (room_id) REFERENCES room(room_id)
);

CREATE TABLE room_subject(
  subject_id SERIAL PRIMARY KEY,
  client_room_id INT NOT NULL,
  room_id INT NOT NULL,
  title VARCHAR(50) UNIQUE NOT NULL,
  elucidation VARCHAR(500) NOT NULL,
  subject_type VARCHAR(50) NOT NULL,
  FOREIGN KEY (client_room_id) REFERENCES client_to_room(client_room_id),
  FOREIGN KEY (room_id) REFERENCES room(room_id)
);

CREATE TABLE subject_chat(
  chat_id SERIAL PRIMARY KEY,
  subject_id INT NOT NULL,
  client_id INT NOT NULL,
  title VARCHAR(50) UNIQUE NOT NULL,
  elucidation VARCHAR(500) NOT NULL,
  user_message VARCHAR NOT NULL,
  FOREIGN KEY (client_id) REFERENCES client(client_id), 
  FOREIGN KEY (subject_id) REFERENCES room_subject(subject_id)
);

CREATE TABLE user_to_user(  
  user_to_user_id SERIAL PRIMARY KEY,
  user_id INT,
  users_chat_id INT,
  FOREIGN KEY (user_id) REFERENCES client(client_id),
  FOREIGN KEY (users_chat_id) REFERENCES subject_chat(chat_id)
);

CREATE TABLE user_to_user_room(
  user_to_user_chat_id SERIAL PRIMARY KEY,
  subject_id INT NOT NULL,
  client_id INT NOT NULL,
  FOREIGN KEY (client_id) REFERENCES client(client_id), 
  FOREIGN KEY (subject_id) REFERENCES user_to_user(user_to_user_id)
);

CREATE TABLE user_to_user_message(
  user_to_user_message_id SERIAL PRIMARY KEY,
  user_to_user_chat_id INT NOT NULL,
  client_id INT NOT NULL,
  user_message VARCHAR NOT NULL,
  FOREIGN KEY (client_id) REFERENCES client(client_id), 
  FOREIGN KEY (user_to_user_chat_id) REFERENCES user_to_user_room(user_to_user_chat_id)
);
