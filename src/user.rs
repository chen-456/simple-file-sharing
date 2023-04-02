use anyhow::Context;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use sodiumoxide::crypto::pwhash::argon2id13 as sodium;

use crate::models::{NewUser, User};

const OPS_LIMIT: sodium::OpsLimit = sodium::OPSLIMIT_INTERACTIVE;
const MEM_LIMIT: sodium::MemLimit = sodium::MemLimit(4 << 20);

fn pwhash(password: &str) -> sodium::HashedPassword {
    sodium::pwhash(password.as_bytes(), OPS_LIMIT, MEM_LIMIT).expect("cannot allocate memory")
}

fn pwhash_as_str(pwhash: &sodium::HashedPassword) -> &str {
    std::str::from_utf8(&pwhash.0)
        .unwrap()
        .trim_end_matches(0 as char)
}

fn pwhash_verify(plain_pass: &str, hashed_pass: &str) -> anyhow::Result<()> {
    let mut hashed_pass_sodium = sodium::HashedPassword([0; sodium::HASHEDPASSWORDBYTES]);
    hashed_pass_sodium.0[..hashed_pass.len()].copy_from_slice(hashed_pass.as_bytes());

    if sodium::pwhash_verify(&hashed_pass_sodium, plain_pass.as_bytes()) {
        Ok(())
    } else {
        Err(anyhow::anyhow!("password incorrect"))
    }
}

pub fn login(
    input_username: &str,
    input_password: &str,
    db: &mut SqliteConnection,
) -> anyhow::Result<i32> {
    use crate::schema::users::dsl::*;

    // Query database
    let records: Vec<User> = users
        .filter(username.eq(input_username))
        .limit(1)
        .load(db)
        .context("query database")?;
    anyhow::ensure!(
        records.len() == 1,
        "user {:?} does not exist",
        input_username,
    );

    // Verify stored password
    pwhash_verify(
        input_password,
        records[0]
            .hashed_pass
            .as_ref()
            .context("the user has disabled password authentication")?,
    )
    .context("verify password")?;

    Ok(records[0].id)
}

pub fn register(
    input_username: &str,
    input_password: &str,
    db: &mut SqliteConnection,
) -> anyhow::Result<i32> {
    use crate::schema::users::dsl::*;

    // Ensure that the user did not exist (as a dual fail-safe)
    let records: Vec<User> = users
        .filter(username.eq(input_username))
        .limit(1)
        .load(db)
        .context("query database")?;
    anyhow::ensure!(
        records.is_empty(),
        "user {:?} already exists",
        input_username,
    );

    // Insert into the database
    let input_pass_hashed = pwhash(input_password);
    let user = NewUser {
        username: input_username,
        hashed_pass: pwhash_as_str(&input_pass_hashed),
    };
    // SQLite does not support SQL `RETURNING` clauses, so we have to manually
    // query the newly created user's ID.
    // https://stackoverflow.com/questions/65437001
    diesel::insert_into(users)
        .values(&user)
        .execute(db)
        .context("insert into database")?;

    // Query user ID
    let records: Vec<User> = users
        .filter(username.eq(input_username))
        .limit(1)
        .load(db)
        .context("query database")?;
    anyhow::ensure!(records.len() == 1, "cannot insert into database");

    Ok(records[0].id)
}
