//! Methods for handling passwords. Passwords are combined with random salt and
//! hashed using [sha2::Sha256]. Only the salt and resultant digest is then
//! persisted to the `users` table in the database. Salt comes from UUIDv4,
//! which
//! [is not a secure source of salt](https://stackoverflow.com/a/3596660/13262536