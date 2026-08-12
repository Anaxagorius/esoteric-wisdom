use crate::templates::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    NhiAncientTemplate, NhiAquaticAmphibiansTemplate, NhiAquaticCetaceansTemplate,
    NhiAquaticHydraTemplate, NhiAquaticNommoTemplate, NhiAquaticTemplate, NhiArtificialTemplate,
    NhiConsciousnessTemplate, NhiDocumentationTemplate, NhiEnergyAstralTemplate,
    NhiEnergyEnergyzoaTemplate, NhiEnergyInterdimensionalTemplate, NhiEnergyLightBeingsTemplate,
    NhiEnergyOrbsTemplate, NhiEnergyPlasmaTemplate, NhiEnergyShadowBeingsTemplate,
    NhiEnergyTemplate, NhiEtTemplate, NhiHybridAdamicEvadamicTemplate, NhiHybridElsElTemplate,
    NhiHybridGreyReptilianTemplate, NhiHybridHubridsTemplate, NhiHybridHumanGreyTemplate,
    NhiHybridSassaniTemplate, NhiHybridTemplate, NhiHybridZetaHumansTemplate,
    NhiInsectoidsInsectoidsTemplate, NhiInsectoidsItipuriansTemplate,
    NhiInsectoidsKlermersTemplate, NhiInsectoidsMantidsTemplate, NhiInsectoidsMantoidsTemplate,
    NhiInsectoidsTemplate, NhiInterdimensionalTemplate, NhiOrbsTemplate, NhiOtherTemplate,
    NhiPlasmaTemplate, NhiRaceAlphaCentauriansTemplate, NhiRaceAltairiansTemplate,
    NhiRaceAndromedansTemplate, NhiRaceAnunnakiTemplate, NhiRaceArcturiansTemplate,
    NhiRaceAviansBlueTemplate, NhiRaceAviansGarudaTemplate, NhiRaceAviansHumanoidTemplate,
    NhiRaceAviansTemplate, NhiRaceEbensTemplate, NhiRaceEgarotTemplate, NhiRaceGreysTemplate,
    NhiRaceLyransTemplate, NhiRaceMaitreTemplate, NhiRaceMantidsTemplate, NhiRaceNordicsTemplate,
    NhiRaceOrionGroupTemplate, NhiRacePleiadiansTemplate, NhiRaceProcyonsTemplate,
    NhiRaceReptiliansTemplate, NhiRaceShadowBeingsTemplate, NhiRaceSiriansTemplate,
    NhiRaceSolipsiRaiTemplate, NhiRaceTallWhitesTemplate, NhiRaceTauCetiansTemplate,
    NhiRaceUmmitesTemplate, NhiRaceVegansTemplate, NhiRacesTemplate,
    NhiReptilianAlphaDraconiansTemplate, NhiReptilianDraconiansTemplate,
    NhiReptilianDragonwormsTemplate, NhiReptilianGeneralTemplate, NhiReptilianHydraTemplate,
    NhiReptilianIguanoidsTemplate, NhiReptilianLacertiansTemplate, NhiReptilianNagasTemplate,
    NhiReptilianSerpentBeingsTemplate, NhiTemplate, NhiTricksterTemplate,
    NhiUltraTerrestrialTemplate,
};
use axum::{response::IntoResponse, routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/extraterrestrial", get(extraterrestrial))
        .route("/ultra-terrestrial", get(ultra_terrestrial))
        .route("/interdimensional", get(interdimensional))
        .route("/plasma", get(plasma))
        .route("/orbs", get(orbs))
        .route("/artificial", get(artificial))
        .route("/hybrid", get(hybrid))
        .route("/hybrid/human-grey", get(hybrid_human_grey))
        .route("/hybrid/grey-reptilian", get(hybrid_grey_reptilian))
        .route("/hybrid/adamic-evadamic", get(hybrid_adamic_evadamic))
        .route("/hybrid/els-el", get(hybrid_els_el))
        .route("/hybrid/zeta-humans", get(hybrid_zeta_humans))
        .route("/hybrid/hu-brids", get(hybrid_hu_brids))
        .route("/hybrid/sassani", get(hybrid_sassani))
        .route("/ancient", get(ancient))
        .route("/consciousness", get(consciousness))
        .route("/trickster", get(trickster))
        .route("/aquatic", get(aquatic))
        .route("/aquatic/nommo", get(aquatic_nommo))
        .route("/aquatic/amphibians", get(aquatic_amphibians))
        .route("/aquatic/cetaceans", get(aquatic_cetaceans))
        .route("/aquatic/hydra", get(aquatic_hydra))
        .route("/other", get(other))
        .route("/documentation", get(documentation))
        .route("/races", get(races))
        .route("/races/greys", get(races_greys))
        .route("/races/tall-whites", get(races_tall_whites))
        .route("/races/nordics", get(races_nordics))
        .route("/races/pleiadians", get(races_pleiadians))
        .route("/races/reptilians", get(races_reptilians))
        .route("/races/reptilians/general", get(races_reptilians_general))
        .route(
            "/races/reptilians/draconians",
            get(races_reptilians_draconians),
        )
        .route(
            "/races/reptilians/alpha-draconians",
            get(races_reptilians_alpha_draconians),
        )
        .route(
            "/races/reptilians/iguanoids",
            get(races_reptilians_iguanoids),
        )
        .route(
            "/races/reptilians/lacertians",
            get(races_reptilians_lacertians),
        )
        .route("/races/reptilians/nagas", get(races_reptilians_nagas))
        .route(
            "/races/reptilians/serpent-beings",
            get(races_reptilians_serpent_beings),
        )
        .route(
            "/races/reptilians/dragonworms",
            get(races_reptilians_dragonworms),
        )
        .route("/races/reptilians/hydra", get(races_reptilians_hydra))
        .route("/races/mantids", get(races_mantids))
        .route("/races/avians", get(races_avians))
        .route("/races/avians/blue-avians", get(races_avians_blue_avians))
        .route("/races/avians/garuda", get(races_avians_garuda))
        .route("/races/avians/humanoid", get(races_avians_humanoid))
        .route("/races/maitre", get(races_maitre))
        .route("/races/sirians", get(races_sirians))
        .route("/races/procyons", get(races_procyons))
        .route("/races/arcturians", get(races_arcturians))
        .route("/races/andromedans", get(races_andromedans))
        .route("/races/altairians", get(races_altairians))
        .route("/races/lyrans", get(races_lyrans))
        .route("/races/vegans", get(races_vegans))
        .route("/races/tau-cetians", get(races_tau_cetians))
        .route("/races/orion-group", get(races_orion_group))
        .route("/races/alpha-centaurians", get(races_alpha_centaurians))
        .route("/races/ebens", get(races_ebens))
        .route("/races/ummites", get(races_ummites))
        .route("/races/shadow-beings", get(races_shadow_beings))
        .route("/races/anunnaki", get(races_anunnaki))
        .route("/races/egarot", get(races_egarot))
        .route("/races/solipsi-rai", get(races_solipsi_rai))
        .route("/insectoids", get(insectoids))
        .route("/insectoids/mantids", get(insectoids_mantids))
        .route("/insectoids/insectoids", get(insectoids_insectoids))
        .route("/insectoids/mantoids", get(insectoids_mantoids))
        .route("/insectoids/itipurians", get(insectoids_itipurians))
        .route("/insectoids/klermers", get(insectoids_klermers))
        .route("/energy", get(energy))
        .route("/energy/energyzoa", get(energy_energyzoa))
        .route("/energy/light-beings", get(energy_light_beings))
        .route("/energy/plasma-entities", get(energy_plasma_entities))
        .route("/energy/orbs", get(energy_orbs))
        .route("/energy/shadow-beings", get(energy_shadow_beings))
        .route("/energy/astral-entities", get(energy_astral_entities))
        .route(
            "/energy/interdimensional-intelligences",
            get(energy_interdimensional_intelligences),
        )
}

async fn hub() -> impl IntoResponse {
    HtmlTemplate(NhiTemplate)
}

async fn extraterrestrial() -> impl IntoResponse {
    HtmlTemplate(NhiEtTemplate)
}

async fn ultra_terrestrial() -> impl IntoResponse {
    HtmlTemplate(NhiUltraTerrestrialTemplate)
}

async fn interdimensional() -> impl IntoResponse {
    HtmlTemplate(NhiInterdimensionalTemplate)
}

async fn plasma() -> impl IntoResponse {
    HtmlTemplate(NhiPlasmaTemplate)
}

async fn orbs() -> impl IntoResponse {
    HtmlTemplate(NhiOrbsTemplate)
}

async fn artificial() -> impl IntoResponse {
    HtmlTemplate(NhiArtificialTemplate)
}

async fn hybrid() -> impl IntoResponse {
    HtmlTemplate(NhiHybridTemplate)
}

async fn hybrid_human_grey() -> impl IntoResponse {
    HtmlTemplate(NhiHybridHumanGreyTemplate)
}

async fn hybrid_grey_reptilian() -> impl IntoResponse {
    HtmlTemplate(NhiHybridGreyReptilianTemplate)
}

async fn hybrid_adamic_evadamic() -> impl IntoResponse {
    HtmlTemplate(NhiHybridAdamicEvadamicTemplate)
}

async fn hybrid_els_el() -> impl IntoResponse {
    HtmlTemplate(NhiHybridElsElTemplate)
}

async fn hybrid_zeta_humans() -> impl IntoResponse {
    HtmlTemplate(NhiHybridZetaHumansTemplate)
}

async fn hybrid_hu_brids() -> impl IntoResponse {
    HtmlTemplate(NhiHybridHubridsTemplate)
}

async fn hybrid_sassani() -> impl IntoResponse {
    HtmlTemplate(NhiHybridSassaniTemplate)
}

async fn ancient() -> impl IntoResponse {
    HtmlTemplate(NhiAncientTemplate)
}

async fn consciousness() -> impl IntoResponse {
    HtmlTemplate(NhiConsciousnessTemplate)
}

async fn trickster() -> impl IntoResponse {
    HtmlTemplate(NhiTricksterTemplate)
}

async fn aquatic() -> impl IntoResponse {
    HtmlTemplate(NhiAquaticTemplate)
}

async fn aquatic_nommo() -> impl IntoResponse {
    HtmlTemplate(NhiAquaticNommoTemplate)
}

async fn aquatic_amphibians() -> impl IntoResponse {
    HtmlTemplate(NhiAquaticAmphibiansTemplate)
}

async fn aquatic_cetaceans() -> impl IntoResponse {
    HtmlTemplate(NhiAquaticCetaceansTemplate)
}

async fn aquatic_hydra() -> impl IntoResponse {
    HtmlTemplate(NhiAquaticHydraTemplate)
}

async fn other() -> impl IntoResponse {
    HtmlTemplate(NhiOtherTemplate)
}

async fn documentation() -> impl IntoResponse {
    HtmlTemplate(NhiDocumentationTemplate)
}

async fn races() -> impl IntoResponse {
    HtmlTemplate(NhiRacesTemplate)
}

async fn races_greys() -> impl IntoResponse {
    HtmlTemplate(NhiRaceGreysTemplate)
}

async fn races_tall_whites() -> impl IntoResponse {
    HtmlTemplate(NhiRaceTallWhitesTemplate)
}

async fn races_nordics() -> impl IntoResponse {
    HtmlTemplate(NhiRaceNordicsTemplate)
}

async fn races_pleiadians() -> impl IntoResponse {
    HtmlTemplate(NhiRacePleiadiansTemplate)
}

async fn races_reptilians() -> impl IntoResponse {
    HtmlTemplate(NhiRaceReptiliansTemplate)
}

async fn races_reptilians_general() -> impl IntoResponse {
    HtmlTemplate(NhiReptilianGeneralTemplate)
}

async fn races_reptilians_draconians() -> impl IntoResponse {
    HtmlTemplate(NhiReptilianDraconiansTemplate)
}

async fn races_reptilians_alpha_draconians() -> impl IntoResponse {
    HtmlTemplate(NhiReptilianAlphaDraconiansTemplate)
}

async fn races_reptilians_iguanoids() -> impl IntoResponse {
    HtmlTemplate(NhiReptilianIguanoidsTemplate)
}

async fn races_reptilians_lacertians() -> impl IntoResponse {
    HtmlTemplate(NhiReptilianLacertiansTemplate)
}

async fn races_reptilians_nagas() -> impl IntoResponse {
    HtmlTemplate(NhiReptilianNagasTemplate)
}

async fn races_reptilians_serpent_beings() -> impl IntoResponse {
    HtmlTemplate(NhiReptilianSerpentBeingsTemplate)
}

async fn races_reptilians_dragonworms() -> impl IntoResponse {
    HtmlTemplate(NhiReptilianDragonwormsTemplate)
}

async fn races_reptilians_hydra() -> impl IntoResponse {
    HtmlTemplate(NhiReptilianHydraTemplate)
}

async fn races_mantids() -> impl IntoResponse {
    HtmlTemplate(NhiRaceMantidsTemplate)
}

async fn races_avians() -> impl IntoResponse {
    HtmlTemplate(NhiRaceAviansTemplate)
}

async fn races_avians_blue_avians() -> impl IntoResponse {
    HtmlTemplate(NhiRaceAviansBlueTemplate)
}

async fn races_avians_garuda() -> impl IntoResponse {
    HtmlTemplate(NhiRaceAviansGarudaTemplate)
}

async fn races_avians_humanoid() -> impl IntoResponse {
    HtmlTemplate(NhiRaceAviansHumanoidTemplate)
}

async fn races_maitre() -> impl IntoResponse {
    HtmlTemplate(NhiRaceMaitreTemplate)
}

async fn races_sirians() -> impl IntoResponse {
    HtmlTemplate(NhiRaceSiriansTemplate)
}

async fn races_procyons() -> impl IntoResponse {
    HtmlTemplate(NhiRaceProcyonsTemplate)
}

async fn races_arcturians() -> impl IntoResponse {
    HtmlTemplate(NhiRaceArcturiansTemplate)
}

async fn races_andromedans() -> impl IntoResponse {
    HtmlTemplate(NhiRaceAndromedansTemplate)
}

async fn races_altairians() -> impl IntoResponse {
    HtmlTemplate(NhiRaceAltairiansTemplate)
}

async fn races_lyrans() -> impl IntoResponse {
    HtmlTemplate(NhiRaceLyransTemplate)
}

async fn races_vegans() -> impl IntoResponse {
    HtmlTemplate(NhiRaceVegansTemplate)
}

async fn races_tau_cetians() -> impl IntoResponse {
    HtmlTemplate(NhiRaceTauCetiansTemplate)
}

async fn races_orion_group() -> impl IntoResponse {
    HtmlTemplate(NhiRaceOrionGroupTemplate)
}

async fn races_alpha_centaurians() -> impl IntoResponse {
    HtmlTemplate(NhiRaceAlphaCentauriansTemplate)
}

async fn races_ebens() -> impl IntoResponse {
    HtmlTemplate(NhiRaceEbensTemplate)
}

async fn races_ummites() -> impl IntoResponse {
    HtmlTemplate(NhiRaceUmmitesTemplate)
}

async fn races_shadow_beings() -> impl IntoResponse {
    HtmlTemplate(NhiRaceShadowBeingsTemplate)
}

async fn races_anunnaki() -> impl IntoResponse {
    HtmlTemplate(NhiRaceAnunnakiTemplate)
}

async fn races_egarot() -> impl IntoResponse {
    HtmlTemplate(NhiRaceEgarotTemplate)
}

async fn races_solipsi_rai() -> impl IntoResponse {
    HtmlTemplate(NhiRaceSolipsiRaiTemplate)
}

async fn insectoids() -> impl IntoResponse {
    HtmlTemplate(NhiInsectoidsTemplate)
}

async fn insectoids_mantids() -> impl IntoResponse {
    HtmlTemplate(NhiInsectoidsMantidsTemplate)
}

async fn insectoids_insectoids() -> impl IntoResponse {
    HtmlTemplate(NhiInsectoidsInsectoidsTemplate)
}

async fn insectoids_mantoids() -> impl IntoResponse {
    HtmlTemplate(NhiInsectoidsMantoidsTemplate)
}

async fn insectoids_itipurians() -> impl IntoResponse {
    HtmlTemplate(NhiInsectoidsItipuriansTemplate)
}

async fn insectoids_klermers() -> impl IntoResponse {
    HtmlTemplate(NhiInsectoidsKlermersTemplate)
}

async fn energy() -> impl IntoResponse {
    HtmlTemplate(NhiEnergyTemplate)
}

async fn energy_energyzoa() -> impl IntoResponse {
    HtmlTemplate(NhiEnergyEnergyzoaTemplate)
}

async fn energy_light_beings() -> impl IntoResponse {
    HtmlTemplate(NhiEnergyLightBeingsTemplate)
}

async fn energy_plasma_entities() -> impl IntoResponse {
    HtmlTemplate(NhiEnergyPlasmaTemplate)
}

async fn energy_orbs() -> impl IntoResponse {
    HtmlTemplate(NhiEnergyOrbsTemplate)
}

async fn energy_shadow_beings() -> impl IntoResponse {
    HtmlTemplate(NhiEnergyShadowBeingsTemplate)
}

async fn energy_astral_entities() -> impl IntoResponse {
    HtmlTemplate(NhiEnergyAstralTemplate)
}

async fn energy_interdimensional_intelligences() -> impl IntoResponse {
    HtmlTemplate(NhiEnergyInterdimensionalTemplate)
}
