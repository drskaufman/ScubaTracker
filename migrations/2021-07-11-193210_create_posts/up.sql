CREATE TABLE dive (
  id SERIAL PRIMARY KEY,
  depth FLOAT NOT NULL,
  startingO2 FLOAT NOT NULL,
  endingO2 FLOAT NOT NULL,
  divelocation VARCHAR NOT NULL,
  divedatetime TIMESTAMP NOT NULL,
  temperature FLOAT NOT NULL,
  divedescription TEXT NOT NULL
);
