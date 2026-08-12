use crate::journal::JournalEntry;
use crate::state::{Organization, TimelineEvent};
use askama::Template;
use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

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

#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingTemplate;

#[derive(Template)]
#[template(path = "app_landing.html")]
pub struct AppLandingTemplate;

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
#[template(path = "hinduism.html")]
pub struct HinduismTemplate;

#[derive(Template)]
#[template(path = "judaism.html")]
pub struct JudaismTemplate;

#[derive(Template)]
#[template(path = "mormonism.html")]
pub struct MormonismTemplate;

#[derive(Template)]
#[template(path = "islam.html")]
pub struct IslamTemplate;

#[derive(Template)]
#[template(path = "christianity.html")]
pub struct ChristianityTemplate;

#[derive(Template)]
#[template(path = "old_testament.html")]
pub struct OldTestamentTemplate;

#[derive(Template)]
#[template(path = "new_testament.html")]
pub struct NewTestamentTemplate;

#[derive(Template)]
#[template(path = "king_james_version.html")]
pub struct KingJamesVersionTemplate;

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
#[template(path = "chinese_religion.html")]
pub struct ChineseReligionTemplate;

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
#[template(path = "maya_religion.html")]
pub struct MayaReligionTemplate;

#[derive(Template)]
#[template(path = "olmec_religion.html")]
pub struct OlmecReligionTemplate;

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
#[template(path = "aztec_religion.html")]
pub struct AztecReligionTemplate;

#[derive(Template)]
#[template(path = "greek_pantheon.html")]
pub struct GreekPantheonTemplate<'a> {
    pub deities: &'a [crate::mythologies_cosmologies::GreekDeity],
}

#[derive(Template)]
#[template(path = "greek_deity.html")]
pub struct GreekDeityTemplate<'a> {
    pub deity: &'a crate::mythologies_cosmologies::GreekDeity,
}

#[derive(Template)]
#[template(path = "roman_pantheon.html")]
pub struct RomanPantheonTemplate<'a> {
    pub deities: &'a [crate::mythologies_cosmologies::RomanDeity],
}

#[derive(Template)]
#[template(path = "roman_deity.html")]
pub struct RomanDeityTemplate<'a> {
    pub deity: &'a crate::mythologies_cosmologies::RomanDeity,
}

#[derive(Template)]
#[template(path = "norse_pantheon.html")]
pub struct NorsePantheonTemplate<'a> {
    pub deities: &'a [crate::mythologies_cosmologies::NorseDeity],
}

#[derive(Template)]
#[template(path = "norse_deity.html")]
pub struct NorseDeityTemplate<'a> {
    pub deity: &'a crate::mythologies_cosmologies::NorseDeity,
}

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

#[derive(Template)]
#[template(path = "yoga.html")]
pub struct YogaTemplate;
// Anomalous Inquiry templates
#[derive(Template)]
#[template(path = "uap.html")]
pub struct UapTemplate;

#[derive(Clone, Copy)]
pub struct EncounterSummary {
    pub slug: &'static str,
    pub title: &'static str,
    pub date_label: &'static str,
    pub teaser: &'static str,
}

#[derive(Clone, Copy)]
pub struct EncounterPage {
    pub slug: &'static str,
    pub title: &'static str,
    pub date_label: &'static str,
    pub location: &'static str,
    pub overview: &'static [&'static str],
    pub key_points: &'static [&'static str],
    pub investigation: &'static [&'static str],
    pub interpretations: &'static [&'static str],
    pub legacy: &'static [&'static str],
}

#[derive(Template)]
#[template(path = "uap_encounters.html")]
pub struct UapEncountersTemplate {
    pub entries: Vec<EncounterSummary>,
}

#[derive(Template)]
#[template(path = "uap_encounter_detail.html")]
pub struct UapEncounterDetailTemplate {
    pub entry: EncounterPage,
}

#[derive(Template)]
#[template(path = "nimitz.html")]
pub struct NimitzTemplate;

#[derive(Template)]
#[template(path = "roswell.html")]
pub struct RoswellTemplate;

#[derive(Template)]
#[template(path = "ce_archive.html")]
pub struct CeArchiveTemplate;

#[derive(Template)]
#[template(path = "phoenix_lights.html")]
pub struct PhoenixLightsTemplate;

#[derive(Template)]
#[template(path = "rendlesham.html")]
pub struct RendleshamTemplate;

#[derive(Template)]
#[template(path = "belgian_wave.html")]
pub struct BelgianWaveTemplate;

#[derive(Template)]
#[template(path = "military_encounters.html")]
pub struct MilitaryEncountersTemplate;

#[derive(Template)]
#[template(path = "uap_soviet_chinese.html")]
pub struct UapSovietChineseTemplate;

#[derive(Template)]
#[template(path = "parapsychology.html")]
pub struct ParapsychologyTemplate;

#[derive(Template)]
#[template(path = "esp.html")]
pub struct EspTemplate;

#[derive(Template)]
#[template(path = "ganzfeld.html")]
pub struct GanzfeldTemplate;

#[derive(Template)]
#[template(path = "precognition.html")]
pub struct PrecognitionTemplate;

#[derive(Template)]
#[template(path = "psychokinesis.html")]
pub struct PsychokinesisTemplate;

#[derive(Template)]
#[template(path = "pear_lab.html")]
pub struct PearLabTemplate;

#[derive(Template)]
#[template(path = "mediumship.html")]
pub struct MediumshipTemplate;

#[derive(Template)]
#[template(path = "reincarnation.html")]
pub struct ReincarnationTemplate;

#[derive(Template)]
#[template(path = "terminal_lucidity.html")]
pub struct TerminalLucidityTemplate;

#[derive(Template)]
#[template(path = "kozyrev_mirror.html")]
pub struct KozyrevMirrorTemplate;

#[derive(Template)]
#[template(path = "dream_telepathy.html")]
pub struct DreamTelepathyTemplate;

#[derive(Template)]
#[template(path = "enhanced_senses.html")]
pub struct EnhancedSensesTemplate;

#[derive(Template)]
#[template(path = "mind_brain.html")]
pub struct MindBrainTemplate;

#[derive(Template)]
#[template(path = "altered_states.html")]
pub struct AlteredStatesTemplate;

#[derive(Template)]
#[template(path = "hypnagogic.html")]
pub struct HypnagogicTemplate;

#[derive(Template)]
#[template(path = "lucid_dreaming.html")]
pub struct LucidDreamingTemplate;

#[derive(Template)]
#[template(path = "trauma.html")]
pub struct TraumaTemplate;

#[derive(Template)]
#[template(path = "visionary.html")]
pub struct VisionaryTemplate;

#[derive(Template)]
#[template(path = "dissociation.html")]
pub struct DissociationTemplate;

#[derive(Template)]
#[template(path = "kundalini.html")]
pub struct KundaliniTemplate;

#[derive(Template)]
#[template(path = "trance.html")]
pub struct TranceTemplate;

#[derive(Template)]
#[template(path = "possession.html")]
pub struct PossessionTemplate;

#[derive(Template)]
#[template(path = "obe.html")]
pub struct ObeTemplate;

#[derive(Template)]
#[template(path = "altered_time.html")]
pub struct AlteredTimeTemplate;

#[derive(Template)]
#[template(path = "high_affect.html")]
pub struct HighAffectTemplate;

#[derive(Template)]
#[template(path = "group_consciousness.html")]
pub struct GroupConsciousnessTemplate;

#[derive(Template)]
#[template(path = "psychosis_adjacent.html")]
pub struct PsychosisAdjacentTemplate;

#[derive(Template)]
#[template(path = "contact_states.html")]
pub struct ContactStatesTemplate;

#[derive(Template)]
#[template(path = "ontological.html")]
pub struct OntologicalTemplate;

#[derive(Template)]
#[template(path = "liminal.html")]
pub struct LiminalTemplate;

#[derive(Template)]
#[template(path = "anomalous_gnosis.html")]
pub struct AnomalousGnosisTemplate;

#[derive(Template)]
#[template(path = "survival.html")]
pub struct SurvivalTemplate;

#[derive(Template)]
#[template(path = "nde.html")]
pub struct NdeTemplate;

#[derive(Template)]
#[template(path = "sde.html")]
pub struct SdeTemplate;

#[derive(Template)]
#[template(path = "nhi.html")]
pub struct NhiTemplate;

#[derive(Template)]
#[template(path = "nhi_et.html")]
pub struct NhiEtTemplate;

#[derive(Template)]
#[template(path = "nhi_ultra_terrestrial.html")]
pub struct NhiUltraTerrestrialTemplate;

#[derive(Template)]
#[template(path = "nhi_interdimensional.html")]
pub struct NhiInterdimensionalTemplate;

#[derive(Template)]
#[template(path = "nhi_plasma.html")]
pub struct NhiPlasmaTemplate;

#[derive(Template)]
#[template(path = "nhi_orbs.html")]
pub struct NhiOrbsTemplate;

#[derive(Template)]
#[template(path = "nhi_artificial.html")]
pub struct NhiArtificialTemplate;

#[derive(Template)]
#[template(path = "nhi_hybrid.html")]
pub struct NhiHybridTemplate;

#[derive(Template)]
#[template(path = "nhi_hybrid_human_grey.html")]
pub struct NhiHybridHumanGreyTemplate;

#[derive(Template)]
#[template(path = "nhi_hybrid_grey_reptilian.html")]
pub struct NhiHybridGreyReptilianTemplate;

#[derive(Template)]
#[template(path = "nhi_hybrid_adamic_evadamic.html")]
pub struct NhiHybridAdamicEvadamicTemplate;

#[derive(Template)]
#[template(path = "nhi_hybrid_els_el.html")]
pub struct NhiHybridElsElTemplate;

#[derive(Template)]
#[template(path = "nhi_hybrid_zeta_humans.html")]
pub struct NhiHybridZetaHumansTemplate;

#[derive(Template)]
#[template(path = "nhi_hybrid_hubrids.html")]
pub struct NhiHybridHubridsTemplate;

#[derive(Template)]
#[template(path = "nhi_hybrid_sassani.html")]
pub struct NhiHybridSassaniTemplate;

#[derive(Template)]
#[template(path = "nhi_ancient.html")]
pub struct NhiAncientTemplate;

#[derive(Template)]
#[template(path = "nhi_consciousness.html")]
pub struct NhiConsciousnessTemplate;

#[derive(Template)]
#[template(path = "nhi_trickster.html")]
pub struct NhiTricksterTemplate;

#[derive(Template)]
#[template(path = "nhi_aquatic.html")]
pub struct NhiAquaticTemplate;

#[derive(Template)]
#[template(path = "nhi_aquatic_nommo.html")]
pub struct NhiAquaticNommoTemplate;

#[derive(Template)]
#[template(path = "nhi_aquatic_amphibians.html")]
pub struct NhiAquaticAmphibiansTemplate;

#[derive(Template)]
#[template(path = "nhi_aquatic_cetaceans.html")]
pub struct NhiAquaticCetaceansTemplate;

#[derive(Template)]
#[template(path = "nhi_aquatic_hydra.html")]
pub struct NhiAquaticHydraTemplate;

#[derive(Template)]
#[template(path = "nhi_other.html")]
pub struct NhiOtherTemplate;

#[derive(Template)]
#[template(path = "nhi_documentation.html")]
pub struct NhiDocumentationTemplate;

#[derive(Template)]
#[template(path = "nhi_races.html")]
pub struct NhiRacesTemplate;

#[derive(Template)]
#[template(path = "nhi_race_greys.html")]
pub struct NhiRaceGreysTemplate;

#[derive(Template)]
#[template(path = "nhi_race_tall_whites.html")]
pub struct NhiRaceTallWhitesTemplate;

#[derive(Template)]
#[template(path = "nhi_race_nordics.html")]
pub struct NhiRaceNordicsTemplate;

#[derive(Template)]
#[template(path = "nhi_race_pleiadians.html")]
pub struct NhiRacePleiadiansTemplate;

#[derive(Template)]
#[template(path = "nhi_race_reptilians.html")]
pub struct NhiRaceReptiliansTemplate;

#[derive(Template)]
#[template(path = "nhi_race_mantids.html")]
pub struct NhiRaceMantidsTemplate;

#[derive(Template)]
#[template(path = "nhi_race_avians.html")]
pub struct NhiRaceAviansTemplate;

#[derive(Template)]
#[template(path = "nhi_race_avians_blue.html")]
pub struct NhiRaceAviansBlueTemplate;

#[derive(Template)]
#[template(path = "nhi_race_avians_garuda.html")]
pub struct NhiRaceAviansGarudaTemplate;

#[derive(Template)]
#[template(path = "nhi_race_avians_humanoid.html")]
pub struct NhiRaceAviansHumanoidTemplate;

#[derive(Template)]
#[template(path = "nhi_race_maitre.html")]
pub struct NhiRaceMaitreTemplate;

#[derive(Template)]
#[template(path = "nhi_race_sirians.html")]
pub struct NhiRaceSiriansTemplate;

#[derive(Template)]
#[template(path = "nhi_race_procyons.html")]
pub struct NhiRaceProcyonsTemplate;

#[derive(Template)]
#[template(path = "nhi_race_arcturians.html")]
pub struct NhiRaceArcturiansTemplate;

#[derive(Template)]
#[template(path = "nhi_race_andromedans.html")]
pub struct NhiRaceAndromedansTemplate;

#[derive(Template)]
#[template(path = "nhi_race_altairians.html")]
pub struct NhiRaceAltairiansTemplate;

#[derive(Template)]
#[template(path = "nhi_race_lyrans.html")]
pub struct NhiRaceLyransTemplate;

#[derive(Template)]
#[template(path = "nhi_race_vegans.html")]
pub struct NhiRaceVegansTemplate;

#[derive(Template)]
#[template(path = "nhi_race_tau_cetians.html")]
pub struct NhiRaceTauCetiansTemplate;

#[derive(Template)]
#[template(path = "nhi_race_orion_group.html")]
pub struct NhiRaceOrionGroupTemplate;

#[derive(Template)]
#[template(path = "nhi_race_alpha_centaurians.html")]
pub struct NhiRaceAlphaCentauriansTemplate;

#[derive(Template)]
#[template(path = "nhi_race_ebens.html")]
pub struct NhiRaceEbensTemplate;

#[derive(Template)]
#[template(path = "nhi_race_ummites.html")]
pub struct NhiRaceUmmitesTemplate;

#[derive(Template)]
#[template(path = "nhi_race_shadow_beings.html")]
pub struct NhiRaceShadowBeingsTemplate;

#[derive(Template)]
#[template(path = "nhi_race_anunnaki.html")]
pub struct NhiRaceAnunnakiTemplate;

#[derive(Template)]
#[template(path = "nhi_race_egarot.html")]
pub struct NhiRaceEgarotTemplate;

#[derive(Template)]
#[template(path = "nhi_race_solipsi_rai.html")]
pub struct NhiRaceSolipsiRaiTemplate;

#[derive(Template)]
#[template(path = "nhi_insectoids.html")]
pub struct NhiInsectoidsTemplate;

#[derive(Template)]
#[template(path = "nhi_insectoids_mantids.html")]
pub struct NhiInsectoidsMantidsTemplate;

#[derive(Template)]
#[template(path = "nhi_insectoids_insectoids.html")]
pub struct NhiInsectoidsInsectoidsTemplate;

#[derive(Template)]
#[template(path = "nhi_insectoids_mantoids.html")]
pub struct NhiInsectoidsMantoidsTemplate;

#[derive(Template)]
#[template(path = "nhi_insectoids_itipurians.html")]
pub struct NhiInsectoidsItipuriansTemplate;

#[derive(Template)]
#[template(path = "nhi_insectoids_klermers.html")]
pub struct NhiInsectoidsKlermersTemplate;

#[derive(Template)]
#[template(path = "nhi_energy.html")]
pub struct NhiEnergyTemplate;

#[derive(Template)]
#[template(path = "nhi_energy_energyzoa.html")]
pub struct NhiEnergyEnergyzoaTemplate;

#[derive(Template)]
#[template(path = "nhi_energy_light_beings.html")]
pub struct NhiEnergyLightBeingsTemplate;

#[derive(Template)]
#[template(path = "nhi_energy_plasma.html")]
pub struct NhiEnergyPlasmaTemplate;

#[derive(Template)]
#[template(path = "nhi_energy_orbs.html")]
pub struct NhiEnergyOrbsTemplate;

#[derive(Template)]
#[template(path = "nhi_energy_shadow_beings.html")]
pub struct NhiEnergyShadowBeingsTemplate;

#[derive(Template)]
#[template(path = "nhi_energy_astral.html")]
pub struct NhiEnergyAstralTemplate;

#[derive(Template)]
#[template(path = "nhi_energy_interdimensional.html")]
pub struct NhiEnergyInterdimensionalTemplate;

#[derive(Template)]
#[template(path = "nhi_reptilian_alpha_draconians.html")]
pub struct NhiReptilianAlphaDraconiansTemplate;

#[derive(Template)]
#[template(path = "nhi_reptilian_draconians.html")]
pub struct NhiReptilianDraconiansTemplate;

#[derive(Template)]
#[template(path = "nhi_reptilian_dragonworms.html")]
pub struct NhiReptilianDragonwormsTemplate;

#[derive(Template)]
#[template(path = "nhi_reptilian_general.html")]
pub struct NhiReptilianGeneralTemplate;

#[derive(Template)]
#[template(path = "nhi_reptilian_hydra.html")]
pub struct NhiReptilianHydraTemplate;

#[derive(Template)]
#[template(path = "nhi_reptilian_iguanoids.html")]
pub struct NhiReptilianIguanoidsTemplate;

#[derive(Template)]
#[template(path = "nhi_reptilian_lacertians.html")]
pub struct NhiReptilianLacertiansTemplate;

#[derive(Template)]
#[template(path = "nhi_reptilian_nagas.html")]
pub struct NhiReptilianNagasTemplate;

#[derive(Template)]
#[template(path = "nhi_reptilian_serpent_beings.html")]
pub struct NhiReptilianSerpentBeingsTemplate;

#[derive(Template)]
#[template(path = "cryptozoology.html")]
pub struct CryptozoologyTemplate;

#[derive(Template)]
#[template(path = "crypto_hominid.html")]
pub struct CryptoHominidTemplate;

#[derive(Template)]
#[template(path = "crypto_canid.html")]
pub struct CryptoCanidTemplate;

#[derive(Template)]
#[template(path = "crypto_feline.html")]
pub struct CryptoFelineTemplate;

#[derive(Template)]
#[template(path = "crypto_reptilian.html")]
pub struct CryptoReptilianTemplate;

#[derive(Template)]
#[template(path = "crypto_aquatic.html")]
pub struct CryptoAquaticTemplate;

#[derive(Template)]
#[template(path = "crypto_avian.html")]
pub struct CryptoAvianTemplate;

#[derive(Template)]
#[template(path = "crypto_insectoid.html")]
pub struct CryptoInsectoidTemplate;

#[derive(Template)]
#[template(path = "crypto_hybrid.html")]
pub struct CryptoHybridTemplate;

#[derive(Template)]
#[template(path = "crypto_fossil_survivor.html")]
pub struct CryptoFossilSurvivorTemplate;

#[derive(Template)]
#[template(path = "crypto_regional.html")]
pub struct CryptoRegionalTemplate;

#[derive(Template)]
#[template(path = "crypto_aerial.html")]
pub struct CryptoAerialTemplate;

#[derive(Template)]
#[template(path = "crypto_unknown.html")]
pub struct CryptoUnknownTemplate;

#[derive(Template)]
#[template(path = "paranormal.html")]
pub struct ParanormalTemplate;

#[derive(Template)]
#[template(path = "paranormal_ghosts.html")]
pub struct ParanormalGhostsTemplate;

#[derive(Template)]
#[template(path = "paranormal_vampires.html")]
pub struct ParanormalVampiresTemplate;

#[derive(Template)]
#[template(path = "paranormal_werewolves.html")]
pub struct ParanormalWerewolvesTemplate;

#[derive(Template)]
#[template(path = "paranormal_zombies.html")]
pub struct ParanormalZombiesTemplate;

#[derive(Template)]
#[template(path = "paranormal_demons.html")]
pub struct ParanormalDemonsTemplate;

#[derive(Template)]
#[template(path = "paranormal_witchcraft.html")]
pub struct ParanormalWitchcraftTemplate;

#[derive(Template)]
#[template(path = "paranormal_haunted.html")]
pub struct ParanormalHauntedTemplate;

#[derive(Template)]
#[template(path = "paranormal_exorcism.html")]
pub struct ParanormalExorcismTemplate;

#[derive(Template)]
#[template(path = "conspiracy.html")]
pub struct ConspiracyTemplate;

#[derive(Template)]
#[template(path = "conspiracy_phenomenon.html")]
pub struct ConspiracyPhenomenonTemplate;

#[derive(Template)]
#[template(path = "conspiracy_political.html")]
pub struct ConspiracyPoliticalTemplate;

#[derive(Template)]
#[template(path = "conspiracy_war.html")]
pub struct ConspiracyWarTemplate;

#[derive(Template)]
#[template(path = "conspiracy_religious.html")]
pub struct ConspiracyReligiousTemplate;

#[derive(Template)]
#[template(path = "conspiracy_cultural.html")]
pub struct ConspiracyCulturalTemplate;

#[derive(Template)]
#[template(path = "conspiracy_technological.html")]
pub struct ConspiracyTechnologicalTemplate;

#[derive(Template)]
#[template(path = "conspiracy_economic.html")]
pub struct ConspiracyEconomicTemplate;

#[derive(Template)]
#[template(path = "conspiracy_health.html")]
pub struct ConspiracyHealthTemplate;

#[derive(Template)]
#[template(path = "timeline.html")]
pub struct TimelineTemplate {
    pub events: Vec<TimelineEvent>,
}

#[derive(Template)]
#[template(path = "organizations.html")]
pub struct OrganizationsTemplate {
    pub organizations: Vec<Organization>,
}

#[derive(Template)]
#[template(path = "japanese_religion.html")]
pub struct JapaneseReligionTemplate;

#[derive(Template)]
#[template(path = "uap_personalities.html")]
pub struct UapPersonalitiesTemplate;

#[derive(Template)]
#[template(path = "uap_personality_george_adamski.html")]
pub struct UapPersonalityGeorgeAdamskiTemplate;

#[derive(Template)]
#[template(path = "uap_personality_orfeo_angelucci.html")]
pub struct UapPersonalityOrfeoAngelucciTemplate;

#[derive(Template)]
#[template(path = "uap_personality_kenneth_arnold.html")]
pub struct UapPersonalityKennethArnoldTemplate;

#[derive(Template)]
#[template(path = "uap_personality_stephen_bassett.html")]
pub struct UapPersonalityStephenBassettTemplate;

#[derive(Template)]
#[template(path = "uap_personality_art_bell.html")]
pub struct UapPersonalityArtBellTemplate;

#[derive(Template)]
#[template(path = "uap_personality_robert_bigelow.html")]
pub struct UapPersonalityRobertBigelowTemplate;

#[derive(Template)]
#[template(path = "uap_personality_chris_bledsoe.html")]
pub struct UapPersonalityChrisBledsoeTemplate;

#[derive(Template)]
#[template(path = "uap_personality_dylan_borland.html")]
pub struct UapPersonalityDylanBorlandTemplate;

#[derive(Template)]
#[template(path = "uap_personality_matthew_brown.html")]
pub struct UapPersonalityMatthewBrownTemplate;

#[derive(Template)]
#[template(path = "uap_personality_jim_coral_lorenzen.html")]
pub struct UapPersonalityJimCoralLorenzensTemplate;

#[derive(Template)]
#[template(path = "uap_personality_philip_corso.html")]
pub struct UapPersonalityPhilipCorsoTemplate;

#[derive(Template)]
#[template(path = "uap_personality_ross_coulthart.html")]
pub struct UapPersonalityRossCoulthartTemplate;

#[derive(Template)]
#[template(path = "uap_personality_jeremy_corbell.html")]
pub struct UapPersonalityJeremyCorbellTemplate;

#[derive(Template)]
#[template(path = "uap_personality_eric_davis.html")]
pub struct UapPersonalityEricDavisTemplate;

#[derive(Template)]
#[template(path = "uap_personality_tom_delonge.html")]
pub struct UapPersonalityTomDelongeTemplate;

#[derive(Template)]
#[template(path = "uap_personality_richard_dolan.html")]
pub struct UapPersonalityRichardDolanTemplate;

#[derive(Template)]
#[template(path = "uap_personality_luis_elizondo.html")]
pub struct UapPersonalityLuisElizondoTemplate;

#[derive(Template)]
#[template(path = "uap_personality_david_fravor.html")]
pub struct UapPersonalityDavidFravorTemplate;

#[derive(Template)]
#[template(path = "uap_personality_raymond_fowler.html")]
pub struct UapPersonalityRaymondFowlerTemplate;

#[derive(Template)]
#[template(path = "uap_personality_stanton_friedman.html")]
pub struct UapPersonalityStantonFriedmanTemplate;

#[derive(Template)]
#[template(path = "uap_personality_tim_gallaudet.html")]
pub struct UapPersonalityTimGallaudetTemplate;

#[derive(Template)]
#[template(path = "uap_personality_steven_greer.html")]
pub struct UapPersonalityStevenGreerTemplate;

#[derive(Template)]
#[template(path = "uap_personality_david_grusch.html")]
pub struct UapPersonalityDavidGruschTemplate;

#[derive(Template)]
#[template(path = "uap_personality_ryan_graves.html")]
pub struct UapPersonalityRyanGravesTemplate;

#[derive(Template)]
#[template(path = "uap_personality_charles_halt.html")]
pub struct UapPersonalityCharlesHaltTemplate;

#[derive(Template)]
#[template(path = "uap_personality_betty_hill.html")]
pub struct UapPersonalityBettyHillTemplate;

#[derive(Template)]
#[template(path = "uap_personality_barney_hill.html")]
pub struct UapPersonalityBarneyHillTemplate;

#[derive(Template)]
#[template(path = "uap_personality_budd_hopkins.html")]
pub struct UapPersonalityBuddHopkinsTemplate;

#[derive(Template)]
#[template(path = "uap_personality_linda_moulton_howe.html")]
pub struct UapPersonalityLindaMoultonHoweTemplate;

#[derive(Template)]
#[template(path = "uap_personality_j_allen_hynek.html")]
pub struct UapPersonalityJAllenHynekTemplate;

#[derive(Template)]
#[template(path = "uap_personality_david_jacobs.html")]
pub struct UapPersonalityDavidJacobsTemplate;

#[derive(Template)]
#[template(path = "uap_personality_leslie_kean.html")]
pub struct UapPersonalityLeslieKeanTemplate;

#[derive(Template)]
#[template(path = "uap_personality_john_keel.html")]
pub struct UapPersonalityJohnKeelTemplate;

#[derive(Template)]
#[template(path = "uap_personality_donald_keyhoe.html")]
pub struct UapPersonalityDonaldKeyhoeTemplate;

#[derive(Template)]
#[template(path = "uap_personality_george_knapp.html")]
pub struct UapPersonalityGeorgeKnappTemplate;

#[derive(Template)]
#[template(path = "uap_personality_bob_lazar.html")]
pub struct UapPersonalityBobLazarTemplate;

#[derive(Template)]
#[template(path = "uap_personality_avi_loeb.html")]
pub struct UapPersonalityAviLoebTemplate;

#[derive(Template)]
#[template(path = "uap_personality_john_mack.html")]
pub struct UapPersonalityJohnMackTemplate;

#[derive(Template)]
#[template(path = "uap_personality_james_mcdonald.html")]
pub struct UapPersonalityJamesMcdonaldTemplate;

#[derive(Template)]
#[template(path = "uap_personality_billy_meier.html")]
pub struct UapPersonalityBillyMeierTemplate;

#[derive(Template)]
#[template(path = "uap_personality_christopher_mellon.html")]
pub struct UapPersonalityChristopherMellonTemplate;

#[derive(Template)]
#[template(path = "uap_personality_edgar_mitchell.html")]
pub struct UapPersonalityEdgarMitchellTemplate;

#[derive(Template)]
#[template(path = "uap_personality_garry_nolan.html")]
pub struct UapPersonalityGarryNolanTemplate;

#[derive(Template)]
#[template(path = "uap_personality_nick_pope.html")]
pub struct UapPersonalityNickPopeTemplate;

#[derive(Template)]
#[template(path = "uap_personality_hal_puthoff.html")]
pub struct UapPersonalityHalPuthoffTemplate;

#[derive(Template)]
#[template(path = "uap_personality_edward_ruppelt.html")]
pub struct UapPersonalityEdwardRuppeltTemplate;

#[derive(Template)]
#[template(path = "uap_personality_robert_salas.html")]
pub struct UapPersonalityRobertSalasTemplate;

#[derive(Template)]
#[template(path = "uap_personality_jim_semivan.html")]
pub struct UapPersonalityJimSemivanTemplate;

#[derive(Template)]
#[template(path = "uap_personality_whitley_strieber.html")]
pub struct UapPersonalityWhitleyStrieberTemplate;

#[derive(Template)]
#[template(path = "uap_personality_leonard_stringfield.html")]
pub struct UapPersonalityLeonardStringfieldTemplate;

#[derive(Template)]
#[template(path = "uap_personality_giorgio_tsoukalos.html")]
pub struct UapPersonalityGiorgioTsoukalosTemplate;

#[derive(Template)]
#[template(path = "uap_personality_jacques_vallee.html")]
pub struct UapPersonalityJacquesValleeTemplate;

#[derive(Template)]
#[template(path = "uap_personality_travis_walton.html")]
pub struct UapPersonalityTravisWaltonTemplate;

#[derive(Template)]
#[template(path = "uap_personality_erich_von_daniken.html")]
pub struct UapPersonalityErichVonDanikenTemplate;
