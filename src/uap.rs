use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    EncounterPage, EncounterSummary,
    BelgianWaveTemplate, CeArchiveTemplate, MilitaryEncountersTemplate, NimitzTemplate,
    PhoenixLightsTemplate, RendleshamTemplate, RoswellTemplate, UapSovietChineseTemplate,
    UapEncounterDetailTemplate, UapEncountersTemplate, UapTemplate,
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
use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

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
        .route("/encounters", get(encounters_hub))
        .route("/encounters/:slug", get(encounter_detail))
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

async fn encounters_hub() -> impl IntoResponse {
    HtmlTemplate(UapEncountersTemplate {
        entries: encounter_summaries(),
    })
}

async fn encounter_detail(Path(slug): Path<String>) -> impl IntoResponse {
    match encounter_by_slug(&slug) {
        Some(entry) => HtmlTemplate(UapEncounterDetailTemplate { entry }).into_response(),
        None => (StatusCode::NOT_FOUND, "Encounter not found").into_response(),
    }
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

const ENCOUNTER_PAGES: &[EncounterPage] = &[
    EncounterPage {
        slug: "nuremberg-1561",
        title: "Nuremberg Celestial Phenomenon",
        date_label: "April 14, 1561",
        location: "Nuremberg, Holy Roman Empire",
        overview: &[
            "At dawn, residents of Nuremberg reported a prolonged aerial spectacle involving spheres, crosses, rods, and dark spear-like forms appearing to maneuver overhead before many seemed to fall toward the horizon. The event is known mainly through a contemporaneous broadsheet printed by Hans Glaser.",
            "Because this case predates modern aviation by centuries, it is often cited in long-duration UFO historiography as an example of recurrent sky-anomaly reporting, while historians also caution that early modern celestial prodigy literature blended observation, theology, and moral interpretation.",
        ],
        key_points: &[
            "Primary source: Hans Glaser illustrated broadsheet, with a narrative framing the event as a divine warning.",
            "Witness descriptions include many small objects emerging from larger cylindrical forms and engaging in apparent aerial conflict.",
            "A large black triangular or spear-like object was also reported in the woodcut narrative.",
            "Reports stated that objects appeared to descend or burn out outside the city after the display.",
        ],
        investigation: &[
            "No scientific investigation in the modern sense occurred; documentation survives through print culture and later historical analysis of prodigy pamphlets.",
            "Modern historians of science compare the account with known atmospheric optics, sunrise effects, and symbolic visual conventions common to sixteenth-century illustrated reporting.",
        ],
        interpretations: &[
            "Mainstream historical interpretations favor atmospheric phenomena and culturally shaped perception under apocalyptic religious frameworks.",
            "UFO-oriented interpretations treat the consistency of motion language in the account as suggestive of an observed physical event later encoded in period symbolism.",
        ],
        legacy: &[
            "The Nuremberg case remains one of the most cited pre-modern sky anomaly records in both skeptical and anomalistic literature.",
            "It is frequently paired with the 1566 Basel reports as a comparative early-modern episode.",
        ],
    },
    EncounterPage {
        slug: "basel-1566",
        title: "Basel Celestial Phenomenon",
        date_label: "July-August 1566",
        location: "Basel, Swiss Confederacy",
        overview: &[
            "Chronicles and broadsheets from Basel describe repeated dawn and dusk appearances of unusual dark and bright spheres in the sky, with language suggesting movement, transformation, and conflict-like behavior.",
            "Like Nuremberg, the Basel reports are embedded in a religious and prophetic context, making source criticism central to evaluating whether the accounts reflect atmospheric events, collective interpretation, or embellished reportage.",
        ],
        key_points: &[
            "Multiple printed reports noted black spheres seen before sunrise and red forms seen at sunset.",
            "Narratives described apparent motion and mutual interaction among the objects.",
            "Accounts framed the event as an omen during a period of confessional and political tension in Europe.",
            "The event entered later anomalous catalogs as a historical parallel to mass-sighting waves.",
        ],
        investigation: &[
            "Primary records are sparse and mediated by print traditions; there is no surviving instrument data or official technical inquiry.",
            "Historians analyze iconography, publication timing, and textual overlap with other celestial prodigy reports of the era.",
        ],
        interpretations: &[
            "Conventional readings emphasize halo phenomena, low-sun optical effects, and narrative amplification through pamphlet markets.",
            "Anomalous interpretations argue that repeated witness language across reports suggests a real, unusual sky event beyond simple symbolic invention.",
        ],
        legacy: &[
            "Basel is frequently treated as part of a paired sixteenth-century anomaly tradition with Nuremberg.",
            "The case is used in debates about how to handle pre-scientific testimony in longitudinal UFO databases.",
        ],
    },
    EncounterPage {
        slug: "great-airship-wave-1896-1897",
        title: "Great Airship Wave",
        date_label: "1896-1897",
        location: "United States (multi-state wave)",
        overview: &[
            "From late 1896 through spring 1897, U.S. newspapers carried hundreds of reports of mysterious 'airships' with lights, structured hulls, and in some cases human-like pilots. Sightings clustered first in California and then spread across the Midwest and South.",
            "The wave emerged before routine powered flight, which made the reports culturally explosive and helped define key motifs later seen in twentieth-century UFO narratives: structured craft, close approaches, occupants, and disputed press credibility.",
        ],
        key_points: &[
            "Reports described cigar-shaped or elongated craft with bright searchlights and mechanical sounds.",
            "Several accounts claimed landings and conversations with 'inventors' or unusual aeronauts.",
            "Coverage intensity was amplified by competitive newspaper publishing and sensational reporting practices.",
            "The wave peaked in 1897 with many reports in Texas, Kansas, and Illinois.",
        ],
        investigation: &[
            "There was no centralized government inquiry; later historians examined newspaper archives, editorial patterns, and repeating narrative templates.",
            "Researchers distinguish likely hoaxes and satire pieces from reports that appear to be straightforward witness statements.",
        ],
        interpretations: &[
            "Skeptical analysis emphasizes media contagion, folklore construction, and speculative misidentification of celestial objects.",
            "Anomalous research notes that some geographically separated reports share unusual operational details not easily explained by one hoax source.",
        ],
        legacy: &[
            "The wave is considered a foundational pre-UFO modern mass sighting event in North America.",
            "It directly frames the context for the Aurora, Texas incident narrative in April 1897.",
        ],
    },
    EncounterPage {
        slug: "aurora-texas-1897",
        title: "Aurora, Texas Airship Incident",
        date_label: "April 17, 1897",
        location: "Aurora, Texas, United States",
        overview: &[
            "A local newspaper reported that an unknown airship struck a windmill in Aurora, exploded, and left debris and the remains of a small pilot described as 'not of this world.' The story rapidly became one of the most famous episodes of the 1897 wave.",
            "Later retellings expanded the narrative to include burial in a local cemetery, alleged recovered metallic fragments, and claims of official removal of evidence, though documentation remains contested.",
        ],
        key_points: &[
            "Primary source is an article in the Dallas Morning News by S.E. Haydon.",
            "Claims include debris with unusual markings and a non-human pilot body.",
            "A local cemetery marker and later ground-survey stories became central to the legend.",
            "No contemporaneous forensic archive survives to verify crash materials.",
        ],
        investigation: &[
            "Twentieth-century UFO researchers interviewed descendants and local residents, and attempted site surveys.",
            "Historical review found strong uncertainty about whether the original article reflected satire, embellishment, or a misinterpreted local event.",
        ],
        interpretations: &[
            "Mainstream historians generally classify the incident as folklore amplified by late nineteenth-century press culture.",
            "Believers point to persistent oral tradition and repeated local testimony as evidence that a real unusual event may underlie the story.",
        ],
        legacy: &[
            "Aurora remains one of the earliest U.S. 'crash retrieval' narratives in UFO culture.",
            "It is frequently cited as a precursor myth-template for later crash cases, especially Roswell-era stories.",
        ],
    },
    EncounterPage {
        slug: "foo-fighters-1944-1945",
        title: "Foo Fighters",
        date_label: "1944-1945",
        location: "European and Pacific theaters",
        overview: &[
            "Allied and Axis pilots reported luminous balls or discs that appeared to pace aircraft, maneuver intelligently, and disappear without clear propulsion signatures. The phenomenon was especially reported by night fighter crews.",
            "Because sightings occurred on opposing sides of the war, Foo Fighters became an early military-intelligence puzzle and are often treated as the first modern combat-era UAP wave.",
        ],
        key_points: &[
            "USAAF 415th Night Fighter Squadron reports are among the most cited records.",
            "Witnesses described glowing orbs executing abrupt motion changes around aircraft.",
            "No definitive adversary weapons program matched the reported behavior postwar.",
            "Reports appear in mission debriefs and wartime intelligence discussion.",
        ],
        investigation: &[
            "Military analysts initially considered enemy secret technology and then downgraded that explanation as no matching systems were confirmed.",
            "Postwar UFO scholarship integrated Foo Fighters into early precursor case catalogs used by later U.S. Air Force projects.",
        ],
        interpretations: &[
            "Conventional explanations include St. Elmo's fire, plasma effects, visual illusions, and stress under combat conditions.",
            "Anomalous researchers emphasize multi-nation pilot testimony and repeated maneuver descriptions as grounds for unresolved classification.",
        ],
        legacy: &[
            "Foo Fighters established a pattern of trained military observers reporting unexplained aerial phenomena.",
            "The case family is still referenced in discussions of pilot reporting stigma and wartime sensor limitations.",
        ],
    },
    EncounterPage {
        slug: "ghost-rockets-1946",
        title: "Ghost Rockets",
        date_label: "1946",
        location: "Sweden, Finland, and Norway",
        overview: &[
            "In 1946, Scandinavia experienced a wave of reports of fast, rocket-like objects crossing the sky, often without expected explosion signatures. Many sightings occurred over lakes and remote northern terrain.",
            "The timing, shortly after World War II and amid emerging Cold War tensions, led to immediate suspicion of Soviet or captured German missile testing.",
        ],
        key_points: &[
            "Thousands of sightings were reported, with several from military personnel and civil aviation observers.",
            "Witnesses often described cigar-like objects with tails or fiery exhaust.",
            "Some reports claimed impacts into lakes with little recoverable debris.",
            "Swedish authorities implemented reporting controls and intelligence filtering.",
        ],
        investigation: &[
            "Swedish military intelligence and defense agencies conducted formal inquiries, collecting reports and tracking trajectories.",
            "No recovered hardware conclusively identified a missile source for the core unexplained subset.",
        ],
        interpretations: &[
            "Conventional views include meteors, bolides, and misreported military exercises combined with rumor expansion.",
            "Residual unexplained cases remain important in European UFO historiography due to volume and official attention.",
        ],
        legacy: &[
            "Ghost Rockets are a central pre-1947 UAP wave in government records.",
            "They helped shape later Nordic military reporting procedures for anomalous aerial events.",
        ],
    },
    EncounterPage {
        slug: "maury-island-1947",
        title: "Maury Island Incident",
        date_label: "June 21, 1947",
        location: "Puget Sound, Washington, United States",
        overview: &[
            "Harold Dahl reported seeing multiple disc-like objects over Puget Sound, one of which allegedly shed metallic slag-like debris that injured his son and killed a dog. The case quickly drew attention from early civilian investigators.",
            "It is one of the earliest postwar U.S. disc cases and became entangled in controversy after military intelligence involvement and the fatal crash of two Army Air Forces officers returning from the region.",
        ],
        key_points: &[
            "Reported debris event occurred from a vessel near Maury Island.",
            "Ray Palmer and Kenneth Arnold became connected to case circulation.",
            "AAF officers Frank Brown and William Davidson died in a B-25 crash carrying case-related materials.",
            "The FBI and military considered possible hoax and security implications.",
        ],
        investigation: &[
            "Army Air Forces Counterintelligence and FBI files treated the case as suspicious and likely fabricated in parts.",
            "Documentation confirms official concern, but evidence quality and witness reliability issues remained severe.",
        ],
        interpretations: &[
            "Most historians classify Maury Island primarily as a hoax or embellished event.",
            "Some researchers continue to argue that official sensitivity and associated deaths justify retaining it in early UAP chronologies.",
        ],
        legacy: &[
            "The incident illustrates how quickly media, intelligence, and folklore converged in the 1947 flying disc era.",
            "It remains a methodological caution case in UFO historical research.",
        ],
    },
    EncounterPage {
        slug: "kenneth-arnold-1947",
        title: "Kenneth Arnold Sighting",
        date_label: "June 24, 1947",
        location: "Near Mount Rainier, Washington, United States",
        overview: &[
            "Private pilot Kenneth Arnold reported nine bright objects moving at high speed near Mount Rainier while searching for a downed aircraft. His account launched the modern flying saucer era in U.S. media.",
            "Arnold described skipping motion 'like a saucer would if you skipped it across water'; this phrasing was transformed by headlines into the term 'flying saucer,' which then spread globally.",
        ],
        key_points: &[
            "Arnold was an experienced pilot with known aviation familiarity.",
            "Objects were described as crescent or disc-like and flying in echelon-like formation.",
            "Estimated speeds appeared beyond known aircraft capabilities for 1947.",
            "The report triggered a nationwide surge in copycat and independent sightings.",
        ],
        investigation: &[
            "Early military projects (Sign and later Grudge/Blue Book) treated Arnold's report as a key baseline case in the opening wave.",
            "No conclusive instrument corroboration survived, making witness credibility central to historical assessment.",
        ],
        interpretations: &[
            "Proposed explanations include mirage-like visual effects, birds, snow reflections, or misjudged aircraft.",
            "Supporters emphasize Arnold's coherent narrative and the immediate alignment with numerous independent reports in the same period.",
        ],
        legacy: &[
            "The Arnold sighting is widely considered the start of the modern UFO period.",
            "It directly set the media and investigative context for Roswell and subsequent 1947 incidents.",
        ],
    },
    EncounterPage {
        slug: "mantell-1948",
        title: "Mantell Incident",
        date_label: "January 7, 1948",
        location: "Kentucky, United States",
        overview: &[
            "Captain Thomas F. Mantell, Kentucky Air National Guard pilot, died when his F-51D Mustang crashed after pursuing an unidentified object reported near Fort Knox. The case became one of the first fatal U.S. military UFO incidents.",
            "The object was described as bright and large at high altitude; Mantell climbed without adequate oxygen equipment and ultimately lost consciousness.",
        ],
        key_points: &[
            "Multiple ground observers, including military personnel, reported the object before interception.",
            "Mantell's communications indicated he was still climbing in pursuit before contact was lost.",
            "His aircraft crashed near Franklin, Kentucky.",
            "The incident received major national press coverage and policy attention.",
        ],
        investigation: &[
            "Early Air Force evaluations considered Skyhook balloon misidentification among plausible explanations.",
            "Project Sign used the case in early threat-assessment files due to pilot fatality and public impact.",
        ],
        interpretations: &[
            "Most analysts now regard a high-altitude balloon and physiological pilot impairment as the most probable explanation.",
            "Debate persists over whether all witness descriptions fit balloon behavior at the time of observation.",
        ],
        legacy: &[
            "Mantell influenced how military authorities framed safety risks tied to UFO intercept attempts.",
            "The case remains a landmark in U.S. Air Force UFO history because of the fatal outcome.",
        ],
    },
    EncounterPage {
        slug: "gorman-dogfight-1948",
        title: "Gorman Dogfight",
        date_label: "October 1, 1948",
        location: "Fargo, North Dakota, United States",
        overview: &[
            "USAF pilot Lieutenant George Gorman engaged a bright light in a prolonged aerial maneuvering encounter above Fargo. He described repeated turns, climbs, and aggressive near-passes unlike conventional aircraft behavior.",
            "The event was observed by additional witnesses from the ground and by personnel at the local airport, making it one of the most discussed early fighter-intercept cases.",
        ],
        key_points: &[
            "Gorman flew an F-51 Mustang and attempted to close on the object for roughly twenty-seven minutes.",
            "Object was described as a white light executing sharp vertical and lateral motion.",
            "No collision occurred, but Gorman reported a near head-on pass.",
            "Ground witnesses corroborated an unusual luminous object in the area.",
        ],
        investigation: &[
            "Air Force analysis eventually favored a weather balloon explanation for core observations.",
            "Case files nevertheless recorded pilot conviction that the object's maneuvering exceeded balloon-like motion.",
        ],
        interpretations: &[
            "Skeptical interpretation centers on night-flying perceptual distortion while tracking a lit balloon or celestial reference.",
            "Pro-UFO readings emphasize pursuit duration and trained pilot reporting under controlled mission conditions.",
        ],
        legacy: &[
            "The Gorman case became a template for discussing dogfight-style UAP encounters.",
            "It remains a standard comparison case for later military visual-only intercept reports.",
        ],
    },
    EncounterPage {
        slug: "lubbock-lights-1951",
        title: "Lubbock Lights",
        date_label: "August-September 1951",
        location: "Lubbock, Texas, United States",
        overview: &[
            "Residents and scientists in Lubbock reported repeated nighttime formations of bluish-white lights crossing the sky in V-like arrays. The events included sightings by Texas Tech professors and photographic captures by local witnesses.",
            "The case gained credibility from repeated observations across dates and a mix of civilian and technically trained observers.",
        ],
        key_points: &[
            "Multiple Texas Tech faculty members independently reported similar formations.",
            "Carl Hart Jr. photographed light formations, producing widely circulated images.",
            "Lights appeared to move rapidly in ordered groups rather than random scatter.",
            "Sightings occurred over several weeks, not a one-night event.",
        ],
        investigation: &[
            "Project Blue Book reviewed witness accounts and photographs.",
            "Air Force analysts suggested possible birds reflecting city lights for some sightings.",
        ],
        interpretations: &[
            "Bird-reflection hypotheses explain certain geometry and brightness changes but remain disputed for all observations.",
            "UFO researchers classify the best-documented nights as unresolved formation-light events.",
        ],
        legacy: &[
            "The Lubbock Lights remain one of Blue Book's most cited mass-formation sighting cases.",
            "The photographs became enduring visual artifacts in early UFO literature.",
        ],
    },
    EncounterPage {
        slug: "washington-dc-flap-1952",
        title: "Washington, D.C. UFO Flap",
        date_label: "July 19-27, 1952",
        location: "Washington, D.C., United States",
        overview: &[
            "Radar operators at Washington National Airport and Andrews Air Force Base tracked unidentified targets over the capital region during multiple nights, with concurrent visual reports from pilots and controllers.",
            "The incidents prompted interceptor scrambles and one of the most significant official UFO press responses in U.S. history, including a high-profile Pentagon press conference.",
        ],
        key_points: &[
            "Primary waves occurred on July 19-20 and July 26-27.",
            "Radar returns showed intermittent rapid movements and stationarity over restricted airspace.",
            "Commercial and military pilots reported lights and anomalous objects during vectoring.",
            "Air Force launched Operation Blue Book-related analyses and public messaging.",
        ],
        investigation: &[
            "USAF attributed much of the radar activity to temperature inversion effects causing anomalous propagation.",
            "Internal and external commentators noted that inversion did not explain every visual and radar correlation claim.",
        ],
        interpretations: &[
            "Mainstream explanation: radar artifacts plus normal air traffic and stars under heightened alert conditions.",
            "Residual-case advocates argue that multi-radar and pilot timing overlap leaves a core unexplained subset.",
        ],
        legacy: &[
            "The flap heavily influenced U.S. policy treatment of public UFO concern in the early Cold War.",
            "It remains a benchmark for radar-visual mass-incursion debates.",
        ],
    },
    EncounterPage {
        slug: "rb-47-1957",
        title: "RB-47 Incident",
        date_label: "July 17, 1957",
        location: "Southern United States (multi-state flight path)",
        overview: &[
            "An RB-47 electronic reconnaissance aircraft crew reported and electronically tracked an anomalous source while flying across the southern U.S., with corroboration from ground radar at points during the event.",
            "The case is notable for combined human observation and onboard electronic countermeasure receiver detections interpreted by some analysts as one of the strongest instrumented military UFO cases.",
        ],
        key_points: &[
            "Crew included trained electronic warfare operators monitoring signal behavior.",
            "Anomalous source appeared to maneuver relative to aircraft and periodically vanish/reappear.",
            "Ground radar and flight crew observations reportedly aligned at intervals.",
            "Event lasted long enough to generate substantial after-action documentation.",
        ],
        investigation: &[
            "Case entered Blue Book and was later reanalyzed by researchers including those in scientific UFO reviews.",
            "Debate focuses on whether ECM receiver data could be attributed to known emitters, propagation effects, or instrumentation artifacts.",
        ],
        interpretations: &[
            "Conventional analyses cite signal ambiguity and complex radar environments in Cold War airspace.",
            "UFO proponents emphasize that multiple sensor channels and trained crew reporting make this case unusually resilient to simple dismissal.",
        ],
        legacy: &[
            "RB-47 remains a cornerstone example in arguments for multi-sensor UAP evidentiary standards.",
            "It is often compared with modern Navy cases where sensor fusion is central.",
        ],
    },
    EncounterPage {
        slug: "antonio-vilas-boas-1957",
        title: "Antonio Vilas-Boas Encounter",
        date_label: "October 16, 1957",
        location: "Minas Gerais, Brazil",
        overview: &[
            "Brazilian farmer Antonio Vilas-Boas reported being taken aboard a craft while working at night, describing medical-like procedures and contact with humanoid occupants. The account became one of the earliest widely known abduction narratives.",
            "His testimony was recorded by physician and UFO researcher João Martins and later circulated internationally, shaping later abduction case frameworks.",
        ],
        key_points: &[
            "Witness described bright object descent in rural farmland and subsequent physical interaction.",
            "Reported symptoms after event included nausea, weakness, and skin irritation.",
            "Case predated the better-known Hill abduction by several years.",
            "Narrative included specific interior craft details and nonverbal communication claims.",
        ],
        investigation: &[
            "Documentation relied primarily on witness testimony and interviews rather than independent instrumentation.",
            "Medical observations of temporary symptoms were cited, though diagnostic specificity was limited.",
        ],
        interpretations: &[
            "Skeptical interpretations include sleep disruption, stress, confabulation, and culturally shaped storytelling.",
            "Believer interpretations view timeline precedence and detailed narrative structure as significant in abduction research history.",
        ],
        legacy: &[
            "The case became foundational in global abduction literature.",
            "It is frequently analyzed for motif continuity with later close-encounter reports.",
        ],
    },
    EncounterPage {
        slug: "betty-barney-hill-1961",
        title: "Betty and Barney Hill Abduction",
        date_label: "September 19-20, 1961",
        location: "New Hampshire, United States",
        overview: &[
            "Betty and Barney Hill reported a close encounter while driving at night and later claimed missing time and onboard experiences recovered under hypnosis. Their case became the most influential early abduction report in the English-speaking world.",
            "Public release of their testimony, psychiatric records discussions, and media coverage transformed the cultural visibility of abduction narratives.",
        ],
        key_points: &[
            "Initial observation involved a bright object following their vehicle.",
            "Witnesses described anxiety, memory gaps, and unusual physical traces on car and clothing.",
            "Hypnosis sessions with Dr. Benjamin Simon produced detailed recall narratives.",
            "Betty Hill's star map account became a major, disputed element.",
        ],
        investigation: &[
            "Case was documented through interviews, hypnosis transcripts, and independent retellings.",
            "No decisive physical evidence resolved core claims, but consistency patterns and psychological records sustained long-term interest.",
        ],
        interpretations: &[
            "Critics emphasize hypnosis suggestibility, media influence, and memory construction effects.",
            "Supporters argue for independent consistency between both witnesses and lasting psychological impact as evidence of a profound event.",
        ],
        legacy: &[
            "The Hills established the modern abduction narrative template in mainstream culture.",
            "Their case remains central in debates on memory reliability and anomalous experience research.",
        ],
    },
    EncounterPage {
        slug: "socorro-1964",
        title: "Socorro Incident",
        date_label: "April 24, 1964",
        location: "Socorro, New Mexico, United States",
        overview: &[
            "Police officer Lonnie Zamora reported a landed egg-shaped craft and small figures near Socorro before the object departed with flame and roar. The event produced physical traces and immediate law-enforcement documentation.",
            "J. Allen Hynek and Project Blue Book investigators treated Socorro as one of the strongest close-encounter cases in official U.S. files.",
        ],
        key_points: &[
            "Zamora observed object at close range while on patrol.",
            "Ground traces included impressions and scorched vegetation.",
            "Witness was considered credible by both local authorities and federal investigators.",
            "No known aircraft fit full observational profile at the reported location/time.",
        ],
        investigation: &[
            "Blue Book conducted site and witness analysis, including trace photography and interview work.",
            "Despite attempted conventional scenarios, no consensus official resolution satisfied all case features.",
        ],
        interpretations: &[
            "Conventional proposals include hoax, experimental craft, or misperception under stress, though each has weaknesses.",
            "UFO researchers rank Socorro as a high-credibility CE2-style event.",
        ],
        legacy: &[
            "Socorro remains one of the best-known police-witness UFO incidents.",
            "It is consistently cited in evidence-focused historical shortlists.",
        ],
    },
    EncounterPage {
        slug: "exeter-1965",
        title: "Exeter Incident",
        date_label: "September 3, 1965",
        location: "Exeter, New Hampshire, United States",
        overview: &[
            "Teen witness Norman Muscarello and responding police officers reported a low-flying, silent object with flashing red lights over rural roads near Exeter. Repeated passovers and multiple witnesses gave the case national attention.",
            "The event became one of the defining New England UFO incidents of the 1960s and was later covered in detail in landmark UFO books.",
        ],
        key_points: &[
            "Muscarello reported pursuit-like behavior by a bright object near a farmhouse.",
            "Officers Eugene Bertrand and David Hunt reported similar observations.",
            "Witnesses described large dark mass with sequential red lights rather than a simple point source.",
            "Case entered Project Blue Book records.",
        ],
        investigation: &[
            "Blue Book inquiries considered stars, aircraft, and military activity, but unresolved elements remained in public discussion.",
            "Journalistic and later research interviews preserved multiple overlapping witness accounts.",
        ],
        interpretations: &[
            "Skeptics cite possible aircraft-light misidentification and nighttime distance ambiguity.",
            "Proponents stress independent police corroboration and repeated low-altitude passes.",
        ],
        legacy: &[
            "Exeter is a classic U.S. law-enforcement-linked UFO case.",
            "It helped establish a narrative model for multi-witness local flap incidents.",
        ],
    },
    EncounterPage {
        slug: "kecksburg-1965",
        title: "Kecksburg Incident",
        date_label: "December 9, 1965",
        location: "Kecksburg, Pennsylvania, United States",
        overview: &[
            "After a bright fireball was observed across several U.S. states and Canada, residents near Kecksburg reported an acorn-like metallic object in wooded terrain and rapid military response restricting access.",
            "Competing explanations have included meteor event, reentering space debris, and classified recovery operations, but witness narratives of an unusual ground object remain disputed.",
        ],
        key_points: &[
            "Regional fireball observations were independently logged over a wide area.",
            "Local witnesses described military trucks and area cordon shortly after reports.",
            "Object was described as bronze/copper colored with band-like markings.",
            "Long-term FOIA and records disputes fueled continuing controversy.",
        ],
        investigation: &[
            "NASA and military record requests produced partial and sometimes conflicting archival responses.",
            "Researchers compared trajectories with known satellite and bolide data, with no universally accepted closure.",
        ],
        interpretations: &[
            "Conventional analysis often points to meteor/space-junk explanations plus memory accretion.",
            "Anomalous interpretations cite retrieval claims and unresolved documentation gaps.",
        ],
        legacy: &[
            "Kecksburg remains one of the strongest U.S. retrieval-claim controversies after Roswell.",
            "The case illustrates how archival uncertainty can sustain long-duration public dispute.",
        ],
    },
    EncounterPage {
        slug: "westall-1966",
        title: "Westall UFO Encounter",
        date_label: "April 6, 1966",
        location: "Melbourne, Australia",
        overview: &[
            "Students and staff at Westall High School reported a low-flying object descending near nearby paddocks before rapidly departing. The witness count and school setting made it one of Australia's most discussed mass sightings.",
            "Accounts from former students describe multiple objects or one object with accompanying craft, plus unusual official presence shortly afterward.",
        ],
        key_points: &[
            "Witness estimates range from dozens to several hundred.",
            "Object described as silver/gray, disc-like, and capable of rapid acceleration.",
            "Some students reported approaching a disturbed grass area.",
            "Witnesses later reported pressure not to discuss the event publicly.",
        ],
        investigation: &[
            "No full public government dossier has definitively resolved the case.",
            "Documentary projects and retrospective interviews have become the primary evidence channels.",
        ],
        interpretations: &[
            "Skeptical proposals include aircraft tests and rumor amplification in a school environment.",
            "Supporters argue witness scale, duration, and consistent motion descriptions exceed simple misidentification models.",
        ],
        legacy: &[
            "Westall is a central Southern Hemisphere mass-sighting case.",
            "It remains active in Australian historical and documentary UAP research.",
        ],
    },
    EncounterPage {
        slug: "pascagoula-1973",
        title: "Pascagoula Abduction",
        date_label: "October 11, 1973",
        location: "Pascagoula, Mississippi, United States",
        overview: &[
            "Charles Hickson and Calvin Parker reported being taken aboard a craft while fishing by the Pascagoula River. They described entity encounters and traumatic aftereffects.",
            "The case gained enduring attention because law enforcement secretly recorded the men while alone, capturing apparent distress and consistency before they knew they were being monitored.",
        ],
        key_points: &[
            "Witnesses reported bright blue light and paralysis before abduction claim.",
            "Entity descriptions included humanoid forms with unusual skin and movement.",
            "Sheriff's office hidden-room recording became central corroborative evidence.",
            "Subsequent witnesses in region reported related lights and anomalies.",
        ],
        investigation: &[
            "Local law enforcement and civilian researchers conducted immediate interviews.",
            "Polygraph and later follow-up assessments were cited by advocates, though methodological critiques persisted.",
        ],
        interpretations: &[
            "Skeptical frameworks include confabulation, stress, and culturally reinforced abduction narrative development.",
            "Proponents stress early-recorded emotional state and long-term witness consistency.",
        ],
        legacy: &[
            "Pascagoula remains a major U.S. abduction-era case with unusual contemporaneous documentation.",
            "It continues to be cited in studies of witness trauma and anomalous memory reports.",
        ],
    },
    EncounterPage {
        slug: "travis-walton-1975",
        title: "Travis Walton Incident",
        date_label: "November 5, 1975",
        location: "Snowflake, Arizona, United States",
        overview: &[
            "Logging crew members reported seeing a luminous craft in the Apache-Sitgreaves region; Travis Walton approached and was allegedly struck by an energy beam before disappearing for five days.",
            "Walton's reappearance and the crew's repeated interviews made the case one of the most heavily publicized abduction-related incidents in U.S. history.",
        ],
        key_points: &[
            "Seven crew members were present during initial event.",
            "Walton was missing for approximately five days before reappearing disoriented.",
            "Crew polygraph results were widely cited in media and research debates.",
            "Case inspired books, documentaries, and the film 'Fire in the Sky'.",
        ],
        investigation: &[
            "Investigators collected witness statements, search records, and polygraph findings over multiple phases.",
            "Critics highlighted inconsistencies and possible social/economic incentives, while advocates highlighted broad witness agreement on core events.",
        ],
        interpretations: &[
            "Skeptical interpretations include staged disappearance or narrative contamination over time.",
            "Supporters point to multi-witness presence and durability of accounts across decades.",
        ],
        legacy: &[
            "Walton became one of the most recognized names in abduction discourse.",
            "The case remains a key comparative reference for missing-time encounter claims.",
        ],
    },
    EncounterPage {
        slug: "tehran-1976",
        title: "Tehran UFO Incident",
        date_label: "September 19, 1976",
        location: "Tehran, Iran",
        overview: &[
            "Iranian Air Force jets were scrambled after civilian and military observers reported a bright anomalous object over Tehran. Pilots reported weapon and instrument disruption during attempted intercept.",
            "A U.S. Defense Intelligence Agency summary later treated the event as highly significant due to multiple witnesses and reported avionics effects.",
        ],
        key_points: &[
            "First interceptor returned due to malfunction; second pilot reported close interaction.",
            "Pilot described a smaller object separating and approaching his aircraft.",
            "Reported temporary loss of communications and weapons systems near object.",
            "Ground witnesses and tower personnel reported unusual lights concurrently.",
        ],
        investigation: &[
            "Iranian military reporting and U.S. intelligence summaries preserved the core event record.",
            "Case often cited in official-document UFO collections because of defense-report provenance.",
        ],
        interpretations: &[
            "Conventional candidates include bright celestial objects, technical failures, and stress-induced misinterpretation.",
            "Residual-case analysis emphasizes reported simultaneous instrument effects and object behavior.",
        ],
        legacy: &[
            "Tehran is one of the best-known non-Western military intercept UAP cases.",
            "It remains frequently referenced in policy discussions on pilot reporting protocols.",
        ],
    },
    EncounterPage {
        slug: "cash-landrum-1980",
        title: "Cash-Landrum Incident",
        date_label: "December 29, 1980",
        location: "Texas, United States",
        overview: &[
            "Betty Cash, Vickie Landrum, and Colby Landrum reported a brilliant diamond-shaped craft emitting intense heat near Dayton, Texas, followed by numerous military helicopters. The witnesses reported acute physical injuries afterward.",
            "The case became notable for medical claims, alleged military association, and subsequent federal litigation.",
        ],
        key_points: &[
            "Witnesses reported severe heat sensation and temporary vehicle effects.",
            "Accounts included multiple heavy-lift helicopters near the object.",
            "Medical records documented burns, hair loss, and ongoing illness claims.",
            "Witnesses filed suit against the U.S. government alleging military involvement.",
        ],
        investigation: &[
            "Civilian UFO groups and legal teams pursued records and witness corroboration.",
            "Court proceedings found no conclusive proof linking identified military aircraft to the event.",
        ],
        interpretations: &[
            "Skeptics question memory reliability and helicopter count claims under nighttime stress.",
            "Proponents emphasize immediate medical impacts and consistency among core witnesses.",
        ],
        legacy: &[
            "Cash-Landrum is a primary CE2/medical-effects case in U.S. UFO literature.",
            "It remains central to discussions of physical harm claims in UAP reports.",
        ],
    },
    EncounterPage {
        slug: "hudson-valley-wave-1982-1986",
        title: "Hudson Valley UFO Wave",
        date_label: "1982-1986",
        location: "New York and Connecticut, United States",
        overview: &[
            "Thousands of residents in the Hudson Valley reported large structured craft or formations moving slowly and silently at low altitude, often with bright geometric light arrays.",
            "Repeated evenings of reports and varied witness backgrounds made this one of the largest sustained U.S. regional waves of the 1980s.",
        ],
        key_points: &[
            "Witnesses included police officers, pilots, and large numbers of civilians.",
            "Objects were often described as boomerang or rectangular platforms with multiple lights.",
            "Events appeared in clusters over several years rather than a single night.",
            "Some prank activity by ultralight pilots was documented, complicating case sorting.",
        ],
        investigation: &[
            "Civilian researchers compiled extensive witness files and timeline reconstructions.",
            "Investigators separated confirmed hoax/prank nights from residual unexplained reports.",
        ],
        interpretations: &[
            "Conventional analyses attribute many sightings to coordinated light-aircraft pranks and misperception.",
            "Residual-case supporters argue that not all reports fit prank profiles, especially large low-altitude structured-object accounts.",
        ],
        legacy: &[
            "Hudson Valley is a major reference case for long-duration regional sighting waves.",
            "It remains important for methodological work on mixed-data events with both hoaxes and unresolved reports.",
        ],
    },
    EncounterPage {
        slug: "varginha-1996",
        title: "Varginha Incident",
        date_label: "January 1996",
        location: "Varginha, Minas Gerais, Brazil",
        overview: &[
            "Residents in Varginha reported unusual creatures and anomalous lights, followed by claims of military capture operations and hospital-linked secrecy. The case became one of Brazil's most famous UFO incidents.",
            "Narratives include multiple witness groups, military vehicles, and alleged chain-of-custody events involving non-human entities, though official confirmation has remained absent.",
        ],
        key_points: &[
            "Three young women reported seeing a crouched creature with unusual eyes and skin texture.",
            "Reports described unusual military activity and transport operations in the city.",
            "Later testimony alleged contact injuries among personnel connected to handling events.",
            "Case persisted through books, documentaries, and local witness testimony.",
        ],
        investigation: &[
            "Brazilian civilian researchers conducted interviews and attempted records tracing for military and medical corroboration.",
            "Official statements generally denied extraordinary claims, often attributing sightings to misunderstandings or ordinary persons/animals.",
        ],
        interpretations: &[
            "Skeptical explanations include rumor cascades, misidentification, and retrospective narrative inflation.",
            "Supporters argue converging witness lines and unusual official behavior indicate an unresolved core event.",
        ],
        legacy: &[
            "Varginha remains a flagship Latin American encounter case.",
            "It is frequently compared to Roswell-style retrieval narratives in modern global UFO culture.",
        ],
    },
    EncounterPage {
        slug: "ohare-2006",
        title: "O'Hare International Airport Sighting",
        date_label: "November 7, 2006",
        location: "Chicago, Illinois, United States",
        overview: &[
            "United Airlines personnel and airport workers reported a dark, disc-like object hovering above gate C17 at O'Hare before it shot upward and allegedly punched a circular opening in the cloud layer.",
            "Because witnesses included trained aviation staff at a major airport, the case received wide media attention despite lack of radar confirmation.",
        ],
        key_points: &[
            "Witnesses included ramp workers, supervisors, pilots, and mechanics.",
            "Object was described as metallic gray and silently hovering for several minutes.",
            "Rapid vertical departure and cloud-hole effect were repeatedly reported.",
            "FAA and airline public responses downplayed event as weather phenomenon.",
        ],
        investigation: &[
            "No formal NTSB-style deep investigation was publicly released for the event.",
            "Journalistic investigations and witness interviews remain the primary documentation sources.",
        ],
        interpretations: &[
            "Skeptical readings include hole-punch clouds plus misperception under uncertain visibility.",
            "Anomalous interpretations emphasize multi-witness airport-professional consistency and structured-object description.",
        ],
        legacy: &[
            "O'Hare became a modern civilian-airport benchmark case.",
            "It is often cited in calls for standardized FAA/UAP reporting pathways.",
        ],
    },
    EncounterPage {
        slug: "stephenville-2008",
        title: "Stephenville Lights",
        date_label: "January 8, 2008",
        location: "Stephenville, Texas, United States",
        overview: &[
            "Dozens of witnesses near Stephenville reported bright lights and large fast-moving objects, with some describing coordinated movement over ranchland and highways. The case escalated after conflicting military statements.",
            "Radar analyses by civilian groups later argued that tracked objects and fighter-jet activity supported a significant unresolved event.",
        ],
        key_points: &[
            "Witnesses included pilots, law enforcement, and longtime local residents.",
            "Initial military explanation denied flights, later revised to acknowledge training missions.",
            "Some reports described very large silent craft with bright perimeter lights.",
            "Case generated one of the larger 2000s-era U.S. media cycles outside coastal military contexts.",
        ],
        investigation: &[
            "MUFON and independent analysts reviewed witness timelines, radar data, and flight logs.",
            "Interpretive disputes focused on matching radar tracks to known aircraft and training operations.",
        ],
        interpretations: &[
            "Conventional analyses attribute sightings to military aircraft, atmospheric effects, and distance misjudgment.",
            "Proponents maintain that specific high-speed tracks and object-size reports remain unexplained.",
        ],
        legacy: &[
            "Stephenville helped bridge older mass-sighting traditions with modern data-rich civilian investigations.",
            "It reinforced public concern over contradictory official messaging in UAP cases.",
        ],
    },
    EncounterPage {
        slug: "aguadilla-2013",
        title: "Aguadilla Incident",
        date_label: "April 25, 2013",
        location: "Aguadilla, Puerto Rico",
        overview: &[
            "A U.S. Customs and Border Protection aircraft captured infrared video of an unidentified object near Rafael Hernandez Airport in Aguadilla. Analysts later claimed the object displayed unusual speed and apparent transmedium behavior.",
            "The case became widely discussed due to video availability and technical reanalysis by independent groups using frame-by-frame modeling.",
        ],
        key_points: &[
            "Event captured by airborne infrared system during border-security mission.",
            "Object appeared to fly low over land, approach water, and continue in maritime zone.",
            "Some analyses interpreted apparent splitting into two objects late in sequence.",
            "Official attribution remains publicly unresolved at high confidence.",
        ],
        investigation: &[
            "Independent technical teams produced trajectory and speed estimates from video metadata.",
            "Counter-analyses argued parallax and camera geometry can explain portions of apparent anomalous motion.",
        ],
        interpretations: &[
            "Skeptical interpretation favors distant airborne object plus sensor perspective effects.",
            "Pro-anomaly interpretations argue remaining transmedium-like behavior is difficult to reconcile with ordinary craft.",
        ],
        legacy: &[
            "Aguadilla is a leading modern FLIR-era analytic case outside the better-known Navy trilogy.",
            "It remains a standard comparison for methods in video-only UAP assessment.",
        ],
    },
    EncounterPage {
        slug: "gimbal-gofast-2014-2015",
        title: "Gimbal and GoFast Encounters",
        date_label: "2014-2015",
        location: "U.S. East Coast training ranges",
        overview: &[
            "U.S. Navy aviators from carrier air wings reported frequent unknown tracks and recorded two widely known infrared videos later labeled GIMBAL and GOFAST. These events occurred during a broader period of repeated Atlantic-range UAP observations.",
            "The videos became central to modern disclosure after official confirmation by the U.S. Department of Defense and subsequent congressional inquiry.",
        ],
        key_points: &[
            "Pilots reported recurring unknown objects over months, not isolated one-off events.",
            "GIMBAL footage showed a rotating-looking object amid pilot commentary about a flight group.",
            "GOFAST footage captured a low apparent target over ocean with range/sensor readouts visible.",
            "Cases were later discussed in Pentagon and congressional UAP frameworks.",
        ],
        investigation: &[
            "DoD confirmed authenticity of released videos as Navy-recorded UAP footage.",
            "Technical debate continues over target speed, range interpretation, sensor mechanics, and apparent rotation artifacts.",
        ],
        interpretations: &[
            "Conventional analyses propose mundane airborne objects with optical/sensor effects causing dramatic appearance.",
            "Anomalous analyses contend that full classified sensor context may indicate capabilities not visible in public clips alone.",
        ],
        legacy: &[
            "GIMBAL and GOFAST, alongside FLIR1, define the modern public UAP evidence era.",
            "They materially influenced creation and expansion of formal U.S. UAP investigative offices.",
        ],
    },
];

fn encounter_summaries() -> Vec<EncounterSummary> {
    ENCOUNTER_PAGES
        .iter()
        .map(|entry| EncounterSummary {
            slug: entry.slug,
            title: entry.title,
            date_label: entry.date_label,
            teaser: entry.overview[0],
        })
        .collect()
}

fn encounter_by_slug(slug: &str) -> Option<EncounterPage> {
    ENCOUNTER_PAGES.iter().find(|entry| entry.slug == slug).copied()
}
