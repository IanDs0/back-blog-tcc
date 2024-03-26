-- Your SQL goes here
CREATE TABLE "users"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"user_tag" VARCHAR UNIQUE NOT NULL,
	"level" INT4 NOT NULL,
	"email" VARCHAR UNIQUE NOT NULL,
	"password" VARCHAR NOT NULL
);

