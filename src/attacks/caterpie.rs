use crate::{
    actions::{
        apply_action_helpers::{Mutations, Probabilities},
        mutations::{active_damage_effect_doutcome, active_damage_effect_mutation},
    },
    types::{Card, EnergyType},
    State,
};

/// Caterpie's "Find a" attack.
///
/// Moves a random Grass-type Pokémon from the deck to the player's hand.
/// Requires one Colorless energy to be attached to Caterpie but does not
/// discard it. If no Grass Pokémon are found, the deck is shuffled.
pub(crate) fn caterpie_find_a(
    acting_player: usize,
    state: &State,
) -> (Probabilities, Mutations) {
    let grass_cards: Vec<Card> = state.decks[acting_player]
        .cards
        .iter()
        .filter(|c| c.get_type() == Some(EnergyType::Grass))
        .cloned()
        .collect();

    if grass_cards.is_empty() {
        return active_damage_effect_doutcome(0, |rng, state, action| {
            state.decks[action.actor].shuffle(false, rng);
        });
    }

    let probabilities = vec![1.0 / grass_cards.len() as f64; grass_cards.len()];
    let mutations = grass_cards
        .into_iter()
        .map(|card| {
            active_damage_effect_mutation(0, {
                move |rng, state, action| {
                    let deck = &mut state.decks[action.actor];
                    if let Some(pos) = deck.cards.iter().position(|c| c == &card) {
                        deck.cards.remove(pos);
                        state.hands[action.actor].push(card.clone());
                    }
                    deck.shuffle(false, rng);
                }
            })
        })
        .collect();

    (probabilities, mutations)
}
