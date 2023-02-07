use jsonwebtoken as jwt;
use jwt::{DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Request,
};

use crate::state::AppState;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Auth {
    /// timestamp
    pub exp: i64,
    /// user id
    pub id: i32,

    pub username: String,
}

impl Auth {
    pub fn get_token(secret: &[u8]) -> String {
        let stub_auth = Auth {
            exp: 0,
            id: 0,
            username: "stub".to_string(),
        };
        jwt::encode(
            &Header::default(),
            &stub_auth,
            &EncodingKey::from_secret(secret),
        )
        .expect("failed to encode token")
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let secret = req
            .rocket()
            .state::<AppState>()
            .expect("state not found")
            .secret
            .clone();

        if let Some(authorization) = req.headers().get_one("Authorization") {
            let token = authorization.replace("Bearer ", "");
            let token_data = jwt::decode::<Auth>(
                &token,
                &DecodingKey::from_secret(&secret),
                &Validation::new(jwt::Algorithm::ES256),
            );

            if let Ok(token_data) = token_data {
                return Outcome::Success(token_data.claims);
            }
        }
        Outcome::Failure((rocket::http::Status::Unauthorized, ()))
    }
}
