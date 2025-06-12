use jsonwebtoken::{encode, EncodingKey, Header};

use crate::models::jwt_models::{UserClaims, UserRoomClaims};

use crate::secret_store::get_secret;

pub fn encode_user_token(claims: UserClaims) -> String {
    

    let server_secret = match get_secret("TOKEN_SECRET"){
        Some(token) => token,
        None => "".to_owned()
    };
    
    let token: String = encode(
	&Header::default(),
	&claims,
	&EncodingKey::from_secret(server_secret.as_str().as_ref()),
    ).unwrap();

    return token;
}


pub fn encode_user_room_token(claims: UserRoomClaims) -> String {
    

    let server_secret = match get_secret("TOKEN_SECRET"){
        Some(token) => token,
        None => "".to_owned()
    };
    
    let token: String = encode(
	&Header::default(),
	&claims,
	&EncodingKey::from_secret(server_secret.as_str().as_ref()),
    ).unwrap();

    return token;
}