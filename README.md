# sqgit

A project I created to learn Rust.

The idea of Simple Query Git is to provide SQL-like query language front-end to git.

Currently it is very limited.

Examples of valid queries:

SELECT hash FROM HEAD
SELECT hash FROM bug-432
SELECT hash FROM HEAD LIMIT 5
SELECT hash FROM HEAD LIMIT 5 OFFSET 15

