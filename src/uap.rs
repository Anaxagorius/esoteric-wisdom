use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    BelgianWaveTemplate, CeArchiveTemplate, MilitaryEncountersTemplate, NimitzTemplate,
    PhoenixLightsTemplate, RendleshamTemplate, RoswellTemplate, UapSovietChineseTemplate,
    UapTemplate,
    UapPersonalitiesTemplate,
    UapPersonalityGeorgeAdamskiTemplate, UapPersonalityOrfeoAngelucciTemplate,
    UapPersonalityKennethArnoldTemplate, UapPersonalityStephenBassettTemplate,
    UapPersonalityArtBellTemplate, UapPersonalityRobertBigelowTemplate,
    UapPersonalityChrisBledsoeTemplate, UapPersonalityDylanBorlandTemplate,
    UapPersonalityMatthewBrownTemplate, UapPersonalityJimCoralLorenzensTemplate,
    UapPersonalityPhilipCorsoTemplate, UapPersonalityRossCoulthartTemplate,
    UapPersonalityJeremyCorbellTemplate, UapPersonalityEricDavisTemplate,
    UapPersonalityTomDelongeTemplate, UapPersonalityRichardDolanTemplate,
    UapPersonalityLuisElizondoTemplate, UapPersonalityDavidFravorTemplate,
    UapPersonalityRaymondFowlerTemplate, UapPersonalityStantonFriedmanTemplate,
    UapPersonalityTimGallaudetTemplate, UapPersonalityStevenGreerTemplate,
    UapPersonalityDavidGruschTemplate, UapPersonalityRyanGravesTemplate,
    UapPersonalityCharlesHaltTemplate, UapPersonalityBettyHillTemplate,
    UapPersonalityBarneyHillTemplate, UapPersonalityBuddHopkinsTemplate,
    UapPersonalityLindaMoultonHoweTemplate, UapPersonalityJAllenHynekTemplate,
    UapPersonalityDavidJacobsTemplate, UapPersonalityLeslieKeanTemplate,
    UapPersonalityJohnKeelTemplate, UapPersonalityDonaldKeyhoeTemplate,
    UapPersonalityGeorgeKnappTemplate, UapPersonalityBobLazarTemplate,
    UapPersonalityAviLoebTemplate, UapPersonalityJohnMackTemplate,
    UapPersonalityJamesMcdonaldTemplate, UapPersonalityBillyMeierTemplate,
    UapPersonalityChristopherMellonTemplate, UapPersonalityEdgarMitchellTemplate,
    UapPersonalityGarryNolanTemplate, UapPersonalityNickPopeTemplate,
    UapPersonalityHalPuthoffTemplate, UapPersonalityEdwardRuppeltTemplate,
    UapPersonalityRobertSalasTemplate, UapPersonalityJimSemivanTemplate,
    UapPersonalityWhitleyStrieberTemplate, UapPersonalityLeonardStringfieldTemplate,
    UapPersonalityGiorgioTsoukalosTemplate, UapPersonalityJacquesValleeTemplate,
    UapPersonalityTravisWaltonTemplate, UapPersonalityErichVonDanikenTemplate,
};
use axum::{response::IntoResponse, routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/nimitz", get(nimitz))
        .route("/roswell", get(roswell))
        .route("/ce-archive", get(ce_archive))
        .route("/phoenix-lights", get(phoenix_lights))
        .route("/rendlesham", get(rendlesham))
        .route("/belgian-wave", get(belgian_wave))
        .route("/military-encounters", get(military_encounters))
        .route("/soviet-chinese-military", get(soviet_chinese_military))
        .route("/personalities", get(personalities_hub))
        .route("/personalities/george-adamski", get(personality_george_adamski))
        .route("/personalities/orfeo-angelucci", get(personality_orfeo_angelucci))
        .route("/personalities/kenneth-arnold", get(personality_kenneth_arnold))
        .route("/personalities/stephen-bassett", get(personality_stephen_bassett))
        .route("/personalities/art-bell", get(personality_art_bell))
        .route("/personalities/robert-bigelow", get(personality_robert_bigelow))
        .route("/personalities/chris-bledsoe", get(personality_chris_bledsoe))
        .route("/personalities/dylan-borland", get(personality_dylan_borland))
        .route("/personalities/matthew-brown", get(personality_matthew_brown))
        .route("/personalities/jim-coral-lorenzen", get(personality_jim_coral_lorenzen))
        .route("/personalities/philip-corso", get(personality_philip_corso))
        .route("/personalities/ross-coulthart", get(personality_ross_coulthart))
        .route("/personalities/jeremy-corbell", get(personality_jeremy_corbell))
        .route("/personalities/eric-davis", get(personality_eric_davis))
        .route("/personalities/tom-delonge", get(personality_tom_delonge))
        .route("/personalities/richard-dolan", get(personality_richard_dolan))
        .route("/personalities/luis-elizondo", get(personality_luis_elizondo))
        .route("/personalities/david-fravor", get(personality_david_fravor))
        .route("/personalities/raymond-fowler", get(personality_raymond_fowler))
        .route("/personalities/stanton-friedman", get(personality_stanton_friedman))
        .route("/personalities/tim-gallaudet", get(personality_tim_gallaudet))
        .route("/personalities/steven-greer", get(personality_steven_greer))
        .route("/personalities/david-grusch", get(personality_david_grusch))
        .route("/personalities/ryan-graves", get(personality_ryan_graves))
        .route("/personalities/charles-halt", get(personality_charles_halt))
        .route("/personalities/betty-hill", get(personality_betty_hill))
        .route("/personalities/barney-hill", get(personality_barney_hill))
        .route("/personalities/budd-hopkins", get(personality_budd_hopkins))
        .route("/personalities/linda-moulton-howe", get(personality_linda_moulton_howe))
        .route("/personalities/j-allen-hynek", get(personality_j_allen_hynek))
        .route("/personalities/david-jacobs", get(personality_david_jacobs))
        .route("/personalities/leslie-kean", get(personality_leslie_kean))
        .route("/personalities/john-keel", get(personality_john_keel))
        .route("/personalities/donald-keyhoe", get(personality_donald_keyhoe))
        .route("/personalities/george-knapp", get(personality_george_knapp))
        .route("/personalities/bob-lazar", get(personality_bob_lazar))
        .route("/personalities/avi-loeb", get(personality_avi_loeb))
        .route("/personalities/john-mack", get(personality_john_mack))
        .route("/personalities/james-mcdonald", get(personality_james_mcdonald))
        .route("/personalities/billy-meier", get(personality_billy_meier))
        .route("/personalities/christopher-mellon", get(personality_christopher_mellon))
        .route("/personalities/edgar-mitchell", get(personality_edgar_mitchell))
        .route("/personalities/garry-nolan", get(personality_garry_nolan))
        .route("/personalities/nick-pope", get(personality_nick_pope))
        .route("/personalities/hal-puthoff", get(personality_hal_puthoff))
        .route("/personalities/edward-ruppelt", get(personality_edward_ruppelt))
        .route("/personalities/robert-salas", get(personality_robert_salas))
        .route("/personalities/jim-semivan", get(personality_jim_semivan))
        .route("/personalities/whitley-strieber", get(personality_whitley_strieber))
        .route("/personalities/leonard-stringfield", get(personality_leonard_stringfield))
        .route("/personalities/giorgio-tsoukalos", get(personality_giorgio_tsoukalos))
        .route("/personalities/jacques-vallee", get(personality_jacques_vallee))
        .route("/personalities/travis-walton", get(personality_travis_walton))
        .route("/personalities/erich-von-daniken", get(personality_erich_von_daniken))
}


async fn hub() -> impl IntoResponse {
    HtmlTemplate(UapTemplate)
}

async fn nimitz() -> impl IntoResponse {
    HtmlTemplate(NimitzTemplate)
}

async fn roswell() -> impl IntoResponse {
    HtmlTemplate(RoswellTemplate)
}

async fn ce_archive() -> impl IntoResponse {
    HtmlTemplate(CeArchiveTemplate)
}

async fn phoenix_lights() -> impl IntoResponse {
    HtmlTemplate(PhoenixLightsTemplate)
}

async fn rendlesham() -> impl IntoResponse {
    HtmlTemplate(RendleshamTemplate)
}

async fn belgian_wave() -> impl IntoResponse {
    HtmlTemplate(BelgianWaveTemplate)
}

async fn military_encounters() -> impl IntoResponse {
    HtmlTemplate(MilitaryEncountersTemplate)
}

async fn soviet_chinese_military() -> impl IntoResponse {
    HtmlTemplate(UapSovietChineseTemplate)
}

async fn personalities_hub() -> impl IntoResponse {
    HtmlTemplate(UapPersonalitiesTemplate)
}

async fn personality_george_adamski() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityGeorgeAdamskiTemplate)
}

async fn personality_orfeo_angelucci() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityOrfeoAngelucciTemplate)
}

async fn personality_kenneth_arnold() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityKennethArnoldTemplate)
}

async fn personality_stephen_bassett() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityStephenBassettTemplate)
}

async fn personality_art_bell() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityArtBellTemplate)
}

async fn personality_robert_bigelow() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityRobertBigelowTemplate)
}

async fn personality_chris_bledsoe() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityChrisBledsoeTemplate)
}

async fn personality_dylan_borland() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityDylanBorlandTemplate)
}

async fn personality_matthew_brown() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityMatthewBrownTemplate)
}

async fn personality_jim_coral_lorenzen() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityJimCoralLorenzensTemplate)
}

async fn personality_philip_corso() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityPhilipCorsoTemplate)
}

async fn personality_ross_coulthart() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityRossCoulthartTemplate)
}

async fn personality_jeremy_corbell() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityJeremyCorbellTemplate)
}

async fn personality_eric_davis() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityEricDavisTemplate)
}

async fn personality_tom_delonge() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityTomDelongeTemplate)
}

async fn personality_richard_dolan() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityRichardDolanTemplate)
}

async fn personality_luis_elizondo() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityLuisElizondoTemplate)
}

async fn personality_david_fravor() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityDavidFravorTemplate)
}

async fn personality_raymond_fowler() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityRaymondFowlerTemplate)
}

async fn personality_stanton_friedman() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityStantonFriedmanTemplate)
}

async fn personality_tim_gallaudet() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityTimGallaudetTemplate)
}

async fn personality_steven_greer() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityStevenGreerTemplate)
}

async fn personality_david_grusch() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityDavidGruschTemplate)
}

async fn personality_ryan_graves() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityRyanGravesTemplate)
}

async fn personality_charles_halt() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityCharlesHaltTemplate)
}

async fn personality_betty_hill() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityBettyHillTemplate)
}

async fn personality_barney_hill() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityBarneyHillTemplate)
}

async fn personality_budd_hopkins() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityBuddHopkinsTemplate)
}

async fn personality_linda_moulton_howe() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityLindaMoultonHoweTemplate)
}

async fn personality_j_allen_hynek() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityJAllenHynekTemplate)
}

async fn personality_david_jacobs() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityDavidJacobsTemplate)
}

async fn personality_leslie_kean() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityLeslieKeanTemplate)
}

async fn personality_john_keel() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityJohnKeelTemplate)
}

async fn personality_donald_keyhoe() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityDonaldKeyhoeTemplate)
}

async fn personality_george_knapp() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityGeorgeKnappTemplate)
}

async fn personality_bob_lazar() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityBobLazarTemplate)
}

async fn personality_avi_loeb() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityAviLoebTemplate)
}

async fn personality_john_mack() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityJohnMackTemplate)
}

async fn personality_james_mcdonald() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityJamesMcdonaldTemplate)
}

async fn personality_billy_meier() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityBillyMeierTemplate)
}

async fn personality_christopher_mellon() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityChristopherMellonTemplate)
}

async fn personality_edgar_mitchell() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityEdgarMitchellTemplate)
}

async fn personality_garry_nolan() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityGarryNolanTemplate)
}

async fn personality_nick_pope() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityNickPopeTemplate)
}

async fn personality_hal_puthoff() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityHalPuthoffTemplate)
}

async fn personality_edward_ruppelt() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityEdwardRuppeltTemplate)
}

async fn personality_robert_salas() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityRobertSalasTemplate)
}

async fn personality_jim_semivan() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityJimSemivanTemplate)
}

async fn personality_whitley_strieber() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityWhitleyStrieberTemplate)
}

async fn personality_leonard_stringfield() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityLeonardStringfieldTemplate)
}

async fn personality_giorgio_tsoukalos() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityGiorgioTsoukalosTemplate)
}

async fn personality_jacques_vallee() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityJacquesValleeTemplate)
}

async fn personality_travis_walton() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityTravisWaltonTemplate)
}

async fn personality_erich_von_daniken() -> impl IntoResponse {
    HtmlTemplate(UapPersonalityErichVonDanikenTemplate)
}
