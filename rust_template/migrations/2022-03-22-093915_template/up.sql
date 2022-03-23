CREATE TABLE IF NOT EXISTS users(
  id BIGSERIAL NOT NULL,
  username VARCHAR(255),
  name VARCHAR(255),
  password VARCHAR(255),
  phoneNumber VARCHAR(255),
  acceptedCommercial BOOL DEFAULT '0',
  acceptedTerms BOOL NOT NULL DEFAULT '0',
  CONSTRAINT PK_Visitor PRIMARY KEY(id)
);

INSERT INTO users ( username, name, password, phoneNumber, acceptedCommercial, acceptedTerms) VALUES ('mike@mail.com', 'test', '1', '123456789', '0', '1') ON CONFLICT DO NOTHING;
INSERT INTO users ( username, name, password, phoneNumber, acceptedCommercial, acceptedTerms) VALUES ('peter@mail.com', 'test', '1', '123456789', '1', '1') ON CONFLICT DO NOTHING;
INSERT INTO users ( username, name, password, phoneNumber, acceptedCommercial, acceptedTerms) VALUES ('pablo@mail.com', 'test', '1', '123456789', '0', '1') ON CONFLICT DO NOTHING;
INSERT INTO users ( username, name, password, phoneNumber, acceptedCommercial, acceptedTerms) VALUES ('test1@mail.com', 'test', '1', '123456789', '0', '1') ON CONFLICT DO NOTHING;
INSERT INTO users ( username, name, password, phoneNumber, acceptedCommercial, acceptedTerms) VALUES ('test2@mail.com', 'test', '1', '123456789', '1', '1') ON CONFLICT DO NOTHING;
INSERT INTO users ( username, name, password, phoneNumber, acceptedCommercial, acceptedTerms) VALUES ('test3@mail.com', 'test', '1', '123456789', '0', '1') ON CONFLICT DO NOTHING;
INSERT INTO users ( username, name, password, phoneNumber, acceptedCommercial, acceptedTerms) VALUES ( 'test4@mail.com', 'test', '1', '123456789', '0', '1') ON CONFLICT DO NOTHING;
INSERT INTO users ( username, name, password, phoneNumber, acceptedCommercial, acceptedTerms) VALUES ( 'test5@mail.com', 'test', '1', '123456789', '1', '1') ON CONFLICT DO NOTHING;
INSERT INTO users ( username, name, password, phoneNumber, acceptedCommercial, acceptedTerms) VALUES ( 'test6@mail.com', 'test', '1', '123456789', '0', '1') ON CONFLICT DO NOTHING;
INSERT INTO users ( username, name, password, phoneNumber, acceptedCommercial, acceptedTerms) VALUES ( 'test7@mail.com', 'test', '1', '123456789', '0', '1') ON CONFLICT DO NOTHING;
