use askama::Template;

#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingTemplate;

#[derive(Template)]
#[template(path = "auth_login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "auth_signup.html")]
pub struct SignupTemplate {
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "astrology.html")]
pub struct AstrologyTemplate;

#[derive(Template)]
#[template(path = "meditation.html")]
pub struct MeditationTemplate;

#[derive(Template)]
#[template(path = "tarot_draw.html")]
pub struct TarotDrawTemplate<'a> {
    pub card: &'a crate::tarot::TarotCard,
    pub reversed: bool,
}

#[derive(Template)]
#[template(path = "journal_list.html")]
pub struct JournalListTemplate;
