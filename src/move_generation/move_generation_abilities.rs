use crate::{ability_ids::AbilityId, actions::SimpleAction, types::PlayedCard, State};

// Use the new function in the filter method
pub(crate) fn generate_ability_actions(state: &State) -> Vec<SimpleAction> {
    let current_player = state.current_player;
    state
        .enumerate_in_play_pokemon(current_player)
        .filter(can_use_ability)
        .map(|(i, _)| SimpleAction::UseAbility(i))
        .collect()
}

fn can_use_ability((in_play_index, card): &(usize, &PlayedCard)) -> bool {
    if card.card.get_ability().is_none() {
        return false;
    }

    let is_active = *in_play_index == 0;
    let ability = AbilityId::from_pokemon_id(&card.card.get_id()[..]).unwrap_or_else(|| {
        panic!(
            "Ability seems not implemented for card ID: {}",
            card.card.get_id()
        )
    });
    match ability {
        AbilityId::A1007Butterfree => !card.ability_used,
        AbilityId::A1177Weezing => is_active && !card.ability_used,
        AbilityId::A1132Gardevoir => !card.ability_used,
        AbilityId::A3122SolgaleoExRisingRoad => !is_active && !card.ability_used,
        AbilityId::A3a027ShiinoticIlluminate => !card.ability_used,
        AbilityId::A2a071Arceus => false,
        AbilityId::A3b034SylveonExHappyRibbon => false,
        AbilityId::A1020VictreebelFragranceTrap => is_active && !card.ability_used,
    }
}
