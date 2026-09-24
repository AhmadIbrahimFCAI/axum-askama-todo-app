use sqlx::{Error::RowNotFound, PgPool};

use crate::data::errors::DataError;


pub async fn create_user(pool: &PgPool, email: &str, password: &str) -> Result<(), DataError>{
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    let bytea_hash = hashed_password.as_bytes();
    
    /* let query = r#"
        insert into users(email, password_hash)
        values($1, $2);
    "#;
    let result = sqlx::query(query)
        .bind(email).bind(bytea_hash)
        .execute(pool).await?; */
    
    sqlx::query!("insert into users(email, password_hash) values($1, $2);", email, bytea_hash)
        .execute(pool).await.map_err(|err|{
            match err {
                sqlx::Error::Database(e) => {
                    if e.constraint() == Some("users_email_key"){
                        DataError::FailedQuery("This email address is already used".to_string())
                    } else {
                        DataError::Internal(e.to_string())
                    }
                },
                e => DataError::Query(e),
            }
        })?;
    
    Ok(())
}



pub async fn authenticate_user(pool: &PgPool, email: &str, password: &str) -> Result<i64, DataError>{

    let user = sqlx::query!(
        "select id, password_hash from users where email = $1", email)
            .fetch_one(pool).await.map_err(|e| {
                match e {
                    sqlx::Error::RowNotFound => DataError::FailedQuery("Invalid credentials".to_string()),
                    e => DataError::Query(e),
                }
            })?;
    
    let hashed_password = String::from_utf8(user.password_hash)?;
    let valid_password = bcrypt::verify(password, &hashed_password)?;
    
    if !valid_password{
        Err(DataError::FailedQuery("Invalid credentials".to_string()))
    } else {
        Ok(user.id)
    }
}

