use askama::Template;
use crate::journal::JournalEntry;

#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingTemplate;

#[derive(Template)]
#[template(path = "app_landing.html")]
pub struct AppLandingTemplate {
    pub is_admin: bool,
}

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
#[template(path = "admin_login.html")]
pub struct AdminLoginTemplate {
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "admin_change_password.html")]
pub struct AdminChangePasswordTemplate {
    pub error: Option<String>,
    pub forced: bool,
}

#[derive(Template)]
#[template(path = "astrology.html")]
pub struct AstrologyTemplate;

#[derive(Template)]
#[template(path = "meditation.html")]
pub struct MeditationTemplate;

#[derive(Template)]
#[template(path = "tarot_select.html")]
pub struct TarotSelectTemplate;

#[derive(Template)]
#[template(path = "tarot_draw.html")]
pub struct TarotDrawTemplate<'a> {
    pub card: &'a crate::tarot::TarotCard,
    pub reversed: bool,
    pub deck_name: &'a str,
    pub deck_key: &'a str,
}

#[derive(Template)]
#[template(path = "journal_list.html")]
pub struct JournalListTemplate {
    pub is_authenticated: bool,
    pub entries: Vec<JournalEntry>,
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "journal_admin.html")]
pub struct AdminJournalTemplate {
    pub entries: Vec<JournalEntry>,
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "journal_guest.html")]
pub struct GuestJournalTemplate {
    pub entries: Vec<JournalEntry>,
}

#[derive(Template)]
#[template(path = "numerology.html")]
pub struct NumerologyTemplate;

#[derive(Template)]
#[template(path = "crystals.html")]
pub struct CrystalsTemplate;

#[derive(Template)]
#[template(path = "runes.html")]
pub struct RunesTemplate;
