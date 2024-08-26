use jsonwebtoken::{
    decode, encode, get_current_timestamp, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rocket::serde::{
    json::{serde_json::json, Value},
    Deserialize, Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Claims {
    pub sub: String,
    pub permission:String,
    exp: u64,
}

pub fn get_jwt() -> Result<Value, String> {
    // 创建jwt
    let header = Header::new(jsonwebtoken::Algorithm::RS256);

    let exp = get_current_timestamp() + 60 * 60;
    let my_claims = Claims {
        sub: "1234567".to_owned(),
        permission: "admin".to_owned(),
        exp,
    };
    let encoding_key = match EncodingKey::from_rsa_pem(include_bytes!("../private_key.pem")) {
        Ok(key) => key,
        Err(err) => Err(format!("{}", err))?,
    };

    let token = match encode(&header, &my_claims, &encoding_key) {
        Ok(token) => token,
        Err(err) => Err(format!("{}", err))?,
    };
    Ok(json!({
        "json":token,
    }))
}
// Bearer xxxx.yyyy.zzz
pub fn validate(token: &str) -> Option<TokenData<Claims>> {
    let splits = token.split(" ").collect::<Vec<_>>();
    if splits.len() != 2 {
        return None;
    }
    if splits[0] != "Bearer" {
        return None;
    }
    let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
    validation.sub = Some("1234567".to_owned());
    validation.leeway = 0;
    let key = match DecodingKey::from_rsa_pem(include_bytes!("../public_key.pem")) {
        Ok(key) => key,
        Err(_) => return None,
    };
    let token_data = match decode::<Claims>(splits[1], &key, &validation) {
        Ok(token_data) => token_data,
        Err(_) => return None,
    };

    println!("{:#?}", token_data);
    Some(token_data)
}
