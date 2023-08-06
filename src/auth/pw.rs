//! Methods for handling passwords. Passwords are combined with random salt and
//! hashed using [sha2::Sha256]. Only the salt and resultant digest is then
//! persisted to the `users` table in the database. Salt comes from UUIDv4,
//! which
//! [is not a secure source of salt](https://stackoverflow.com/a/3596660/13262536),
//! so please use long and secure passwords, and PRs to fix this are very
//! welcome!

use anyhow::{bail, Result};
use base64::{engine::general_purpose, Engine};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Default)]
pub struct HashedPw {
    pub salt: String,
    pub digest: String,
}

fn hash(pw: &str, salt: &str) -> HashedPw {
    let mut pw_digest = salt.to_string();
    pw_digest.