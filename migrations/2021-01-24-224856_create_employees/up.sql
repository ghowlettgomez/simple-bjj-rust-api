-- Your SQL goes here

CREATE TYPE belt AS ENUM ('white', 'blue', 'purple', 'brown', 'black', 'coral', 'red');

CREATE TABLE "employees" (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    belt belt NOT NULL,
    salary INT NOT NULL,
    age INT NOT NULL
)