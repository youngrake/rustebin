CREATE TABLE IF NOT EXISTS pastes 
(
	id VARCHAR PRIMARY KEY,
	owner VARCHAR references users(id),
	is_url BOOLEAN NOT NULL DEFAULT 'f',
	body TEXT NOT NULL
)
