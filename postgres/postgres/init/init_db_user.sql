CREATE USER java_jtq_user WITH PASSWORD 'admin';
CREATE DATABASE jtq_db;
\connect jtq_db;
CREATE SCHEMA schema_jtq;
SET search_path TO schema_jtq;
GRANT ALL PRIVILEGES ON DATABASE jtq_db to java_jtq_user;