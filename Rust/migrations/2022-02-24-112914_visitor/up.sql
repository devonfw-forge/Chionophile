-- Your SQL goes here
CREATE TABLE IF NOT EXISTS visitors (
	id TEXT PRIMARY KEY NOT NULL,
   	username VARCHAR NOT NULL,
   	name VARCHAR NOT NULL,
   	phone_number VARCHAR NOT NULL,
   	password VARCHAR,
   	accepted_commercial BOOLEAN DEFAULT false,
   	accepted_terms BOOLEAN DEFAULT false,
   	user_type BOOLEAN DEFAULT false
);

CREATE TABLE IF NOT EXISTS queues (
	id TEXT PRIMARY KEY NOT NULL,
   	name VARCHAR,
   	logo VARCHAR,
   	current_number VARCHAR,
   	attention_time TIMESTAMP,
   	min_attention_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   	active BOOLEAN NOT NULL DEFAULT true,
   	customers INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS access_codes (
	id TEXT PRIMARY KEY NOT NULL,
   	ticket_number VARCHAR,
   	creation_time TIMESTAMP,
   	start_time TIMESTAMP,
   	end_time TIMESTAMP,
   	queue_id TEXT,
   	visitor_id TEXT,
   	FOREIGN KEY (queue_id) REFERENCES queues(id),
   	FOREIGN KEY (visitor_id) REFERENCES visitors(id)
);

INSERT INTO visitors (id,  username, name, password, phone_number, accepted_commercial, accepted_terms, user_type) VALUES ('d3a1d9b1-2d3b-460e-9786-d7273d5430e8', 'mike@mail.com', 'test', '1', '123456789', false, true, '1');
INSERT INTO visitors (id,  username, name, password, phone_number, accepted_commercial, accepted_terms, user_type) VALUES ('729df827-2ae3-49df-8705-418e58ea72df',  'peter@mail.com', 'test', '1', '123456789', true, true, '0');
INSERT INTO visitors (id,  username, name, password, phone_number, accepted_commercial, accepted_terms, user_type) VALUES ('809deb78-9f0f-4d91-9494-755329753c2e',  'pablo@mail.com', 'test', '1', '123456789', false, true, '0');
INSERT INTO visitors (id,  username, name, password, phone_number, accepted_commercial, accepted_terms, user_type) VALUES ('1eb340ae-d0e6-4489-88c3-7c64fe03dce4',  'test2@mail.com', 'test', '1', '123456789', false, true, '0');
INSERT INTO visitors (id,  username, name, password, phone_number, accepted_commercial, accepted_terms, user_type) VALUES ('69d81b04-00d3-4fd1-a9f1-da64de8cdb0c',  'test3@mail.com', 'test', '1', '123456789', false, true, '0');
INSERT INTO visitors (id,  username, name, password, phone_number, accepted_commercial, accepted_terms, user_type) VALUES ('3ec96ba8-efa7-4cbc-9ac1-41fd36c83ff5',  'test4@mail.com', 'test', '1', '123456789', false, true, '0');
INSERT INTO visitors (id,  username, name, password, phone_number, accepted_commercial, accepted_terms, user_type) VALUES ('871db513-8ab5-4d3c-a30a-97f5796835cb',  'test5@mail.com', 'test', '1', '123456789', true, true, '0');
INSERT INTO visitors (id,  username, name, password, phone_number, accepted_commercial, accepted_terms, user_type) VALUES ('c8eded73-5193-4032-80f5-ba76ff2f8ddf',  'test6@mail.com', 'test', '1', '123456789', false, true, '0');
INSERT INTO visitors (id,  username, name, password, phone_number, accepted_commercial, accepted_terms, user_type) VALUES ('25e81da4-9103-4fd8-885c-6b538f93bfa7',  'test7@mail.com', 'test', '1', '123456789', false, true, '0');

INSERT INTO queues (id,  name, logo, current_number, attention_time, min_attention_time, active, customers) VALUES ('3bf8544b-67fd-47e9-8bf3-a2a868805f59', 'Day2', 'C:/logos/Day1Logo.png', 'Q001', NULL, '1970-01-01 00:01:00', TRUE, 8);

INSERT INTO access_codes (id,  ticket_number, creation_time, start_time, end_time, visitor_id, queue_id) VALUES ('86c69294-5118-4fc1-97ee-7227f91f4342',  'Q001', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, NULL, 'd3a1d9b1-2d3b-460e-9786-d7273d5430e8', '3bf8544b-67fd-47e9-8bf3-a2a868805f59');
INSERT INTO access_codes (id,  ticket_number, creation_time, start_time, end_time, visitor_id, queue_id) VALUES ('706b4d58-ef05-4720-b2e2-40a56a833138',  'Q002', CURRENT_TIMESTAMP, '2008-01-01 00:00:01', NULL,'729df827-2ae3-49df-8705-418e58ea72df', '3bf8544b-67fd-47e9-8bf3-a2a868805f59');
INSERT INTO access_codes (id,  ticket_number, creation_time, start_time, end_time, visitor_id, queue_id) VALUES ('e12c1348-5fc2-4c63-aff5-20bc7a3b00cc',  'Q003', CURRENT_TIMESTAMP, '2008-01-01 00:00:01', NULL,'809deb78-9f0f-4d91-9494-755329753c2e', '3bf8544b-67fd-47e9-8bf3-a2a868805f59');
INSERT INTO access_codes (id,  ticket_number, creation_time, start_time, end_time, visitor_id, queue_id) VALUES ('bcd76696-2cfb-4826-8ccd-6c543d6fd0d5',  'Q004', CURRENT_TIMESTAMP, '2008-01-01 00:00:01', NULL, '1eb340ae-d0e6-4489-88c3-7c64fe03dce4', '3bf8544b-67fd-47e9-8bf3-a2a868805f59');
INSERT INTO access_codes (id,  ticket_number, creation_time, start_time, end_time, visitor_id, queue_id) VALUES ('629b4f7c-f3dd-43cd-8c2d-a924022d957f',  'Q005', CURRENT_TIMESTAMP, '2008-01-01 00:00:01', NULL, '69d81b04-00d3-4fd1-a9f1-da64de8cdb0c', '3bf8544b-67fd-47e9-8bf3-a2a868805f59');
INSERT INTO access_codes (id,  ticket_number, creation_time, start_time, end_time, visitor_id, queue_id) VALUES ('6209dbec-8404-4a45-8a16-3b0a7b0bbc4e',  'Q006', CURRENT_TIMESTAMP, '2008-01-01 00:00:01', NULL, '3ec96ba8-efa7-4cbc-9ac1-41fd36c83ff5', '3bf8544b-67fd-47e9-8bf3-a2a868805f59');
INSERT INTO access_codes (id,  ticket_number, creation_time, start_time, end_time, visitor_id, queue_id) VALUES ('f97f1208-b313-4f6a-a15d-f2a4997169ac',  'Q007', CURRENT_TIMESTAMP, '2008-01-01 00:00:01', NULL,'871db513-8ab5-4d3c-a30a-97f5796835cb', '3bf8544b-67fd-47e9-8bf3-a2a868805f59');
INSERT INTO access_codes (id,  ticket_number, creation_time, start_time, end_time, visitor_id, queue_id) VALUES ('9954e3cb-5c2a-4ebe-a9da-c25a8b41212c',  'Q008', CURRENT_TIMESTAMP, '2008-01-01 00:00:01', NULL, 'c8eded73-5193-4032-80f5-ba76ff2f8ddf', '3bf8544b-67fd-47e9-8bf3-a2a868805f59');










