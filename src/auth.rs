use axum::{routing::get, Form, Router, extract::State, response::{IntoResponse, Response}, http::{StatusCode, header}};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize as SerdeDeserialize};
use crate::state::{AppState, User, save_users};
use crate::templates::{LoginTemplate, SignupTemplate};

#[derive(Debug, Deserialize)]
pub struct AuthForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, SerdeDeserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(login_page).post(login_post))
        .route("/signup", get(signup_page).post(signup_post))
        .route("/logout", get(logout))
}

pub async fn login_page() -> impl IntoResponse {
    let tpl = LoginTemplate { error: None };
    HtmlTemplate(tpl)
}

pub async fn signup_page() -> impl IntoResponse {
    let tpl = SignupTemplate { error: None };
    HtmlTemplate(tpl)
}

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: askama::Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => (StatusCode::OK, [(header::CONTENT_TYPE, "text/html; charset=utf-8")], html).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

pub async fn signup_post(State(state): State<AppState>, Form(form): Form<AuthForm>) -> impl IntoResponse {
    let mut users = state.users.write().await;
    if users.iter().any(|u| u.email == form.email) {
        let tpl = SignupTemplate { error: Some("Email already registered".into()) };
        return HtmlTemplate(tpl).into_response();
    }

    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(form.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap()
        .to_string();

    let user = User::new(form.email.clone(), password_hash);
    users.push(user);
    let snapshot = users.clone();
    drop(users);
    save_users(&snapshot).await;

    let tpl = LoginTemplate { error: Some("Account created. Please log in.".into()) };
    HtmlTemplate(tpl).into_response()
}

pub async fn login_post(State(state): State<AppState>, cookies: Cookies, Form(form): Form<AuthForm>) -> impl IntoResponse {
    let users = state.users.read().await;
    let user = if let Some(u) = users.iter().find(|u| u.email == form.email) {
        u
    } else {
        let tpl = LoginTemplate { error: Some("Invalid credentials".into()) };
        return HtmlTemplate(tpl).into_response();
    };

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();
    let argon2 = Argon2::default();
    if argon2.verify_password(form.password.as_bytes(), &parsed_hash).is_err() {
        let tpl = LoginTemplate { error: Some("Invalid credentials".into()) };
        return HtmlTemplate(tpl).into_response();
    }

    let exp = chrono::Utc::now().timestamp() as usize + 60 * 60 * 24;
    let claims = Claims { sub: user.id.to_string(), exp };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(state.jwt_secret.as_bytes())).unwrap();

    cookies.add(
        Cookie::build(("esoteric_session", token))
            .path("/")
            .http_only(true)
            .build(),
    );

    (StatusCode::FOUND, [(header::LOCATION, "/".to_string())]).into_response()
}

pub fn decode_token(state: &AppState, token: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .ok()
    .map(|data| data.claims)
}

pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    let mut removal = Cookie::new("esoteric_session", "");
    removal.set_path("/");
    removal.make_removal();
    cookies.add(removal);
    (StatusCode::FOUND, [(header::LOCATION, "/")]).into_response()
}
