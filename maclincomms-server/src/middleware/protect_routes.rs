use std::future::{ready, Ready};

use actix_web::{
    dev::Payload, 
    error::{ErrorGatewayTimeout, ErrorUnauthorized}, 
    http::header::HeaderValue, 
    web, 
    Error, 
    FromRequest, 
    HttpRequest
};

use jsonwebtoken::{
    decode, 
    errors::{Error as JwtError, ErrorKind}, 
    Algorithm, 
    DecodingKey, 
    TokenData, 
    Validation
};

use crate::secret_store::get_secret;



use crate::models::jwt_models::{
    UserAuthenticationTokenPayload, 
    UserClaims, 
    UserRoomAuthenticationTokenPayload, 
    UserRoomClaims
};



impl FromRequest for UserAuthenticationTokenPayload {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
	let req = req.clone();

	let authorization_header_option: Option<&HeaderValue> = req.headers().get(actix_web::http::header::AUTHORIZATION);

	// No Header was sent
	if authorization_header_option.is_none() { return ready(Err(ErrorUnauthorized("No authentication token sent!"))); }

	let authentication_token: String = authorization_header_option.unwrap().to_str().unwrap_or("").to_string();

    
	// Couldn't convert Header::Authorization to String
	if authentication_token.is_empty() { return ready(Err(ErrorUnauthorized("Authentication token has unknown data!"))) }


    let server_secret = match get_secret("TOKEN_SECRET"){
        Some(token) => token,
        None => "".to_owned()
    };



	let token_result: Result<TokenData<UserClaims>, JwtError> = decode::<UserClaims>(
	    &authentication_token,
	    &DecodingKey::from_secret(server_secret.as_str().as_ref()),
	    &Validation::new(Algorithm::HS256),
	);
 

	match token_result {
        Ok(token) => {
            // Wrap the decoded token data into your desired struct
            ready(Ok(UserAuthenticationTokenPayload {
                username: token.claims.username,
            }))
        },
        Err(_e) => ready(Err(ErrorUnauthorized("Invalid Authentication Token!")))
    }
}
}



impl FromRequest for UserRoomAuthenticationTokenPayload {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
	let req = req.clone();

	let authorization_header_option: Option<&HeaderValue> = req.headers().get(actix_web::http::header::AUTHORIZATION);

	// No Header was sent
	if authorization_header_option.is_none() { return ready(Err(ErrorUnauthorized("No authentication token sent!"))); }

	let authentication_token: String = authorization_header_option.unwrap().to_str().unwrap_or("").to_string();

    
	// Couldn't convert Header::Authorization to String
	if authentication_token.is_empty() { return ready(Err(ErrorUnauthorized("Authentication token has unknown data!"))) }


    let server_secret = match get_secret("TOKEN_SECRET"){
        Some(token) => token,
        None => "".to_owned()
    };



	let token_result: Result<TokenData<UserRoomClaims>, JwtError> = decode::<UserRoomClaims>(
	    &authentication_token,
	    &DecodingKey::from_secret(server_secret.as_str().as_ref()),
	    &Validation::new(Algorithm::HS256),
	);
   

	match token_result {
        Ok(token) => {
            // Wrap the decoded token data into your desired struct
            ready(Ok(UserRoomAuthenticationTokenPayload {
                username: token.claims.username,
                room_name: token.claims.room_name,
                room_key: token.claims.room_key,
                role: token.claims.role
            }))
        },
        Err(_e) => ready(Err(ErrorUnauthorized("Invalid Authentication Token!")))
    }
}
}

