create table DailyQueue(
  id BIGINT NOT NULL,
  modificationCounter INTEGER NOT NULL,
  name VARCHAR(255),
  logo VARCHAR(255),
  currentNumber VARCHAR(255),
  attentionTime TIMESTAMP,
  minAttentionTime TIMESTAMP NOT NULL DEFAULT NOW(),
  active BOOL NOT NULL DEFAULT '1',
  customers INTEGER NOT NULL DEFAULT '0',
  CONSTRAINT PK_DailyQueue PRIMARY KEY(id)
);