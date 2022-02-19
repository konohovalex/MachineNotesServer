use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types::Text,
    types::{FromSql, IsNull, ToSql},
    AsExpression, FromSqlRow,
};
use serde::{Deserialize, Serialize};
use std::io::Write;

pub enum PasswordStrengthIssue {
    ContainsWhitespace,
    TooShort,
    TooLong,
    ContainsNoUpperCaseLetter,
    ContainsNoLowerCaseLetter,
    ContainsNoDigit,
    ContainsNoSymbol,
    SymbolUniquenessRequirementsViolation,
    None,
}

#[derive(AsExpression, Debug, FromSqlRow)]
#[sql_type = "Text"]
pub enum HashAlgorithm {
    Argon2idV19,
}

impl ToSql<Text, Pg> for HashAlgorithm {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            HashAlgorithm::Argon2idV19 => out.write_all(b"argon_2_id_v_19")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for HashAlgorithm {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"argon_2_id_v_19" => Ok(HashAlgorithm::Argon2idV19),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

pub struct HashData {
    pub hash: String,
    pub salt: String,
    pub algorithm: HashAlgorithm,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}
