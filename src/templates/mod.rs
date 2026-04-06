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
#[template(path = "ce5.html")]
pub struct Ce5Template;

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

#[derive(Template)]
#[template(path = "middle_eastern_esotericism.html")]
pub struct MiddleEasternEsotericismTemplate;

#[derive(Template)]
#[template(path = "sufism.html")]
pub struct SufismTemplate;

#[derive(Template)]
#[template(path = "zoroastrianism.html")]
pub struct ZoroastrianismTemplate;

#[derive(Template)]
#[template(path = "mandaeism.html")]
pub struct MandaeismTemplate;

#[derive(Template)]
#[template(path = "kemetism.html")]
pub struct KemetismTemplate;

#[derive(Template)]
#[template(path = "mesopotamian_mysteries.html")]
pub struct MesopotamianMysteriesTemplate;

#[derive(Template)]
#[template(path = "merkabah.html")]
pub struct MerkabahTemplate;

#[derive(Template)]
#[template(path = "esoteric_practices.html")]
pub struct EsotericPracticesTemplate;

#[derive(Template)]
#[template(path = "initiation_rites.html")]
pub struct InitiationRitesTemplate;

#[derive(Template)]
#[template(path = "theurgy.html")]
pub struct TheurgyTemplate;

#[derive(Template)]
#[template(path = "hermetic_prayer.html")]
pub struct HermeticPrayerTemplate;

#[derive(Template)]
#[template(path = "invocation_evocation.html")]
pub struct InvocationEvocationTemplate;

#[derive(Template)]
#[template(path = "sacred_geometry.html")]
pub struct SacredGeometryTemplate;

#[derive(Template)]
#[template(path = "mantra_vibration.html")]
pub struct MantraVibrationTemplate;

#[derive(Template)]
#[template(path = "breathwork.html")]
pub struct BreathworkTemplate;

#[derive(Template)]
#[template(path = "astral_travel.html")]
pub struct AstralTravelTemplate;

#[derive(Template)]
#[template(path = "dream_incubation.html")]
pub struct DreamIncubationTemplate;

#[derive(Template)]
#[template(path = "vision_quests.html")]
pub struct VisionQuestsTemplate;

#[derive(Template)]
#[template(path = "i_ching.html")]
pub struct IChingTemplate;

#[derive(Template)]
#[template(path = "geomancy.html")]
pub struct GeomancyTemplate;

#[derive(Template)]
#[template(path = "scrying.html")]
pub struct ScryingTemplate;

#[derive(Template)]
#[template(path = "palmistry.html")]
pub struct PalmistryTemplate;

#[derive(Template)]
#[template(path = "esoteric_meditation.html")]
pub struct EsotericMeditationTemplate;

#[derive(Template)]
#[template(path = "esoteric_astrology.html")]
pub struct EsotericAstrologyTemplate;

#[derive(Template)]
#[template(path = "esoteric_tarot.html")]
pub struct EsotericTarotTemplate;

#[derive(Template)]
#[template(path = "esoteric_runes.html")]
pub struct EsotericRunesTemplate;

#[derive(Template)]
#[template(path = "esoteric_numerology.html")]
pub struct EsotericNumerologyTemplate;

#[derive(Template)]
#[template(path = "esoteric_corpora.html")]
pub struct EsotericCorporaTemplate;

#[derive(Template)]
#[template(path = "corpus_hermeticum.html")]
pub struct CorpusHermeticumTemplate;

#[derive(Template)]
#[template(path = "nag_hammadi.html")]
pub struct NagHammadiTemplate;

#[derive(Template)]
#[template(path = "zohar.html")]
pub struct ZoharTemplate;

#[derive(Template)]
#[template(path = "sefer_yetzirah.html")]
pub struct SeferYetzirahTemplate;

#[derive(Template)]
#[template(path = "sefer_bahir.html")]
pub struct SeferBahirTemplate;

#[derive(Template)]
#[template(path = "emerald_tablet.html")]
pub struct EmeraldTabletTemplate;

#[derive(Template)]
#[template(path = "upanishads_esoteric.html")]
pub struct UpanishadsEsotericTemplate;

#[derive(Template)]
#[template(path = "tantras_texts.html")]
pub struct TantrasTextsTemplate;

#[derive(Template)]
#[template(path = "tibetan_book_dead.html")]
pub struct TibetanBookDeadTemplate;

#[derive(Template)]
#[template(path = "book_of_the_law.html")]
pub struct BookOfTheLawTemplate;

#[derive(Template)]
#[template(path = "picatrix.html")]
pub struct PicatrixTemplate;

#[derive(Template)]
#[template(path = "chaldean_oracles.html")]
pub struct ChaldeaOraclesTemplate;

#[derive(Template)]
#[template(path = "orphic_hymns.html")]
pub struct OrphicHymnsTemplate;

#[derive(Template)]
#[template(path = "orders_societies.html")]
pub struct OrdersSocietiesTemplate;

#[derive(Template)]
#[template(path = "essenes.html")]
pub struct EssenesTemplate;

#[derive(Template)]
#[template(path = "pythagorean_brotherhood.html")]
pub struct PythagoreanBrotherhoodTemplate;

#[derive(Template)]
#[template(path = "eleusinian_mysteries.html")]
pub struct EleusinianMysteriesTemplate;

#[derive(Template)]
#[template(path = "orphic_mysteries.html")]
pub struct OrphicMysteriesTemplate;

#[derive(Template)]
#[template(path = "mithraic_mysteries.html")]
pub struct MithraicMysteriesTemplate;

#[derive(Template)]
#[template(path = "dionysian_mysteries.html")]
pub struct DionysianMysteriesTemplate;

#[derive(Template)]
#[template(path = "egyptian_mystery_schools.html")]
pub struct EgyptianMysterySchoolsTemplate;

#[derive(Template)]
#[template(path = "knights_templar.html")]
pub struct KnightsTemplarTemplate;

#[derive(Template)]
#[template(path = "rosicrucian_orders.html")]
pub struct RosicrucianOrdersTemplate;

#[derive(Template)]
#[template(path = "freemasonry_esoteric.html")]
pub struct FreemasonryEsotericTemplate;

#[derive(Template)]
#[template(path = "martinism.html")]
pub struct MartinismTemplate;

#[derive(Template)]
#[template(path = "illuminism.html")]
pub struct IlluminismTemplate;

#[derive(Template)]
#[template(path = "sufi_orders_esoteric.html")]
pub struct SufiOrdersEsotericTemplate;

#[derive(Template)]
#[template(path = "modern_contemporary_orders.html")]
pub struct ModernContemporaryOrdersTemplate;

#[derive(Template)]
#[template(path = "esoteric_mythologies_cosmologies.html")]
pub struct EsotericMythologiesCosmologiesTemplate;

#[derive(Template)]
#[template(path = "gnostic_aeons.html")]
pub struct GnosticAeonsTemplate;

#[derive(Template)]
#[template(path = "demiurge_myth.html")]
pub struct DemiurgeMythTemplate;

#[derive(Template)]
#[template(path = "sophia_traditions.html")]
pub struct SophiaTraditionsTemplate;

#[derive(Template)]
#[template(path = "tree_of_life_cosmology.html")]
pub struct TreeOfLifeCosmologyTemplate;

#[derive(Template)]
#[template(path = "emanation_theories.html")]
pub struct EmanationTheoriesTemplate;

#[derive(Template)]
#[template(path = "platonic_world_soul.html")]
pub struct PlatonicWorldSoulTemplate;

#[derive(Template)]
#[template(path = "astral_planes.html")]
pub struct AstralPlanesTemplate;

#[derive(Template)]
#[template(path = "chakric_systems.html")]
pub struct ChakricSystemsTemplate;

#[derive(Template)]
#[template(path = "sephirothic_hierarchies.html")]
pub struct SephirothicHierarchiesTemplate;

#[derive(Template)]
#[template(path = "esoteric_figures.html")]
pub struct EsotericFiguresTemplate;

#[derive(Template)]
#[template(path = "hermes_trismegistus.html")]
pub struct HermesTrismegistusTemplate;

#[derive(Template)]
#[template(path = "thoth.html")]
pub struct ThothTemplate;

#[derive(Template)]
#[template(path = "enoch_metatron.html")]
pub struct EnochMetatronTemplate;

#[derive(Template)]
#[template(path = "orpheus.html")]
pub struct OrpheusFigureTemplate;

#[derive(Template)]
#[template(path = "melchizedek.html")]
pub struct MelchizedekTemplate;

#[derive(Template)]
#[template(path = "zoroaster_figure.html")]
pub struct ZoroasterFigureTemplate;

#[derive(Template)]
#[template(path = "pythagoras.html")]
pub struct PythagorasFigureTemplate;

#[derive(Template)]
#[template(path = "plotinus.html")]
pub struct PlatinusTemplate;

#[derive(Template)]
#[template(path = "iamblichus.html")]
pub struct IamblicusTemplate;

#[derive(Template)]
#[template(path = "paracelsus.html")]
pub struct ParacelsusTemplate;

#[derive(Template)]
#[template(path = "marsilio_ficino.html")]
pub struct MarsilisFicinoTemplate;

#[derive(Template)]
#[template(path = "jacob_boehme.html")]
pub struct JacobBohmeTemplate;

#[derive(Template)]
#[template(path = "emanuel_swedenborg.html")]
pub struct EmanuelSwedenborgTemplate;

#[derive(Template)]
#[template(path = "helena_blavatsky.html")]
pub struct HelenaBlavatskyTemplate;

#[derive(Template)]
#[template(path = "rudolf_steiner.html")]
pub struct RudolfSteinerTemplate;

#[derive(Template)]
#[template(path = "dion_fortune.html")]
pub struct DionFortuneTemplate;

#[derive(Template)]
#[template(path = "eliphas_levi.html")]
pub struct EliphasLeviTemplate;

#[derive(Template)]
#[template(path = "papus.html")]
pub struct PapusTemplate;

#[derive(Template)]
#[template(path = "esoteric_concepts_themes.html")]
pub struct EsotericConceptsThemesTemplate;

#[derive(Template)]
#[template(path = "gnosis.html")]
pub struct GnosisTemplate;

#[derive(Template)]
#[template(path = "initiation_concept.html")]
pub struct InitiationConceptTemplate;

#[derive(Template)]
#[template(path = "hidden_wisdom.html")]
pub struct HiddenWisdomTemplate;

#[derive(Template)]
#[template(path = "inner_revelation.html")]
pub struct InnerRevelationTemplate;

#[derive(Template)]
#[template(path = "sacred_secrecy.html")]
pub struct SacredSecrecyTemplate;

#[derive(Template)]
#[template(path = "symbolism_over_literalism.html")]
pub struct SymbolismLiteralismTemplate;

#[derive(Template)]
#[template(path = "microcosm_macrocosm.html")]
pub struct MicrocosmMacrocosmTemplate;

#[derive(Template)]
#[template(path = "as_above_so_below.html")]
pub struct AsAboveSoBelowTemplate;

#[derive(Template)]
#[template(path = "inner_christ_buddha_nature.html")]
pub struct InnerChristBuddhaNatureTemplate;

#[derive(Template)]
#[template(path = "transmutation_consciousness.html")]
pub struct TransmutationConsciousnessTemplate;

#[derive(Template)]
#[template(path = "ascension_illumination.html")]
pub struct AscensionIlluminationTemplate;

#[derive(Template)]
#[template(path = "spiritual_alchemy.html")]
pub struct SpiritualAlchemyTemplate;
