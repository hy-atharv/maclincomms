use jsonwebtoken::{encode, EncodingKey, Header};

use crate::models::jwt_models::UserClaims;

use std::env;


pub fn encode_user_token(claims: UserClaims) -> String {
    dotenvy::dotenv();
    let secret = match env::var("TOKEN_SECRET"){
        Ok(sec) => sec,
        Err(err) => "".to_owned()
    };
    
    let token: String = encode(
	&Header::default(),
	&claims,
	&EncodingKey::from_secret(secret.as_str().as_ref()),
    ).unwrap();

    return token;
}