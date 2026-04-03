use askama::Template;
use crate::journal::JournalEntry;

#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingTemplate;

#[derive(Template)]
#[template(path = "app_landing.html")]
pub struct AppLandingTemplate {
    pub is_admin: bool,
    pub is_user: bool,
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

#[derive(Template)]
#[template(path = "shamanism.html")]
pub struct ShamanismTemplate;

#[derive(Template)]
#[template(path = "druidism.html")]
pub struct DruidismTemplate;

#[derive(Template)]
#[template(path = "akashic_records.html")]
pub struct AkashicRecordsTemplate;

#[derive(Template)]
#[template(path = "wicca.html")]
pub struct WiccaTemplate;

#[derive(Template)]
#[template(path = "gateway_process.html")]
pub struct GatewayProcessTemplate;

#[derive(Template)]
#[template(path = "remote_viewing.html")]
pub struct RemoteViewingTemplate;

#[derive(Template)]
#[template(path = "khasarov_mirror.html")]
pub struct KhasarovMirrorTemplate;

#[derive(Template)]
#[template(path = "alchemy.html")]
pub struct AlchemyTemplate;

#[derive(Template)]
#[template(path = "western_esotericism.html")]
pub struct WesternEsotericismTemplate;

#[derive(Template)]
#[template(path = "hermeticism.html")]
pub struct HermeticismTemplate;

#[derive(Template)]
#[template(path = "gnosticism.html")]
pub struct GnosticismTemplate;

#[derive(Template)]
#[template(path = "neoplatonism.html")]
pub struct NeoplatonismTemplate;

#[derive(Template)]
#[template(path = "rosicrucianism.html")]
pub struct RosicrucianismTemplate;

#[derive(Template)]
#[template(path = "christian_mysticism.html")]
pub struct ChristianMysticismTemplate;

#[derive(Template)]
#[template(path = "theosophy.html")]
pub struct TheosophyTemplate;

#[derive(Template)]
#[template(path = "anthroposophy.html")]
pub struct AnthroposophyTemplate;

#[derive(Template)]
#[template(path = "kabbalah.html")]
pub struct KabbalahTemplate;

#[derive(Template)]
#[template(path = "christian_kabbalah.html")]
pub struct ChristianKabbalahTemplate;

#[derive(Template)]
#[template(path = "occultism.html")]
pub struct OccultismTemplate;

#[derive(Template)]
#[template(path = "eastern_esotericism.html")]
pub struct EasternEsotericismTemplate;

#[derive(Template)]
#[template(path = "tantra.html")]
pub struct TantraTemplate;

#[derive(Template)]
#[template(path = "vajrayana.html")]
pub struct VajrayanaTemplate;

#[derive(Template)]
#[template(path = "dzogchen.html")]
pub struct DzogchenTemplate;

#[derive(Template)]
#[template(path = "mahamudra.html")]
pub struct MahamudraTemplate;

#[derive(Template)]
#[template(path = "taoist_inner_alchemy.html")]
pub struct TaoistInnerAlchemyTemplate;

#[derive(Template)]
#[template(path = "esoteric_confucianism.html")]
pub struct EsotericConfucianismTemplate;

#[derive(Template)]
#[template(path = "shugendo.html")]
pub struct ShugendoTemplate;

#[derive(Template)]
#[template(path = "shingon.html")]
pub struct ShingonTemplate;

#[derive(Template)]
#[template(path = "kashmir_shaivism.html")]
pub struct KashmirShaivismTemplate;

#[derive(Template)]
#[template(path = "siddha.html")]
pub struct SiddhaTemplate;

#[derive(Template)]
#[template(path = "indigenous_esotericism.html")]
pub struct IndigenousEsotericismTemplate;

#[derive(Template)]
#[template(path = "andean_cosmology.html")]
pub struct AndeanCosmologyTemplate;

#[derive(Template)]
#[template(path = "native_american_vision.html")]
pub struct NativeAmericanVisionTemplate;

#[derive(Template)]
#[template(path = "african_initiatory.html")]
pub struct AfricanInitiatoryTemplate;

#[derive(Template)]
#[template(path = "aboriginal_dreamtime.html")]
pub struct AboriginalDreamtimeTemplate;

#[derive(Template)]
#[template(path = "hawaiian_huna.html")]
pub struct HawaiianHunaTemplate;

#[derive(Template)]
#[template(path = "inuit_angakkuq.html")]
pub struct InuitAngakkuqTemplate;
