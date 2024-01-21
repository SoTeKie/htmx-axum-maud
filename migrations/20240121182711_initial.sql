-- Add migration script here
CREATE TABLE articles(
	id uuid PRIMARY KEY,
	title VARCHAR(100) NOT NULL,
	content TEXT NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
