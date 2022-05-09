\connect jtq_db;

CREATE TABLE IF NOT EXISTS Visitor(
  id bigint NOT NULL,
  modificationCounter INTEGER NOT NULL,
  username VARCHAR(255),
  name VARCHAR(255),
  password VARCHAR(255),
  phoneNumber VARCHAR(255),
  acceptedCommercial BOOL DEFAULT '0',
  acceptedTerms BOOL NOT NULL DEFAULT '0',
  userType BOOL DEFAULT '0',
  created_at timestamp without time zone NULL DEFAULT now(),
  CONSTRAINT PK_Visitor PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS DailyQueue (
  id bigint NOT NULL,
  modificationcounter integer NOT NULL,
  name character varying(255) NULL,
  logo character varying(255) NULL,
  currentnumber character varying(255) NULL,
  attentiontime timestamp without time zone NULL,
  minattentiontime timestamp without time zone NOT NULL DEFAULT '2025-01-01 00:00:00':: timestamp without time zone,
  active boolean NOT NULL DEFAULT true,
  customers integer NOT NULL DEFAULT 0,
  created_at timestamp without time zone NULL DEFAULT now(),
  CONSTRAINT pk_dailyqueue PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS AccessCode (
  id bigint NOT NULL,
  modificationcounter integer NOT NULL,
  creationtime timestamp without time zone NULL,
  starttime timestamp without time zone NULL,
  endtime timestamp without time zone NULL,
  idvisitor bigint NOT NULL,
  idqueue bigint NOT NULL,
  created_at timestamp without time zone NULL DEFAULT now(),
  CONSTRAINT pk_accesscode PRIMARY KEY (id)
);

ALTER TABLE Visitor OWNER TO jtq_user;
ALTER TABLE DailyQueue OWNER TO jtq_user;
ALTER TABLE AccessCode OWNER TO jtq_user;
