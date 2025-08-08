use std::collections::HashMap;

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    players::{EndTurnPlayer, Player},
    types::{Card, EnergyType, PlayedCard},
    Deck, Game, State,
};

fn setup_state(deck_a: &Deck, deck_b: &Deck, caterpie_hp: u32, caterpie_card: &Card) -> State {
    let mut state = State::default();
    state.decks[0] = deck_a.clone();
    state.decks[1] = deck_b.clone();
    state.turn_count = 1;
    let active = PlayedCard::new(
        caterpie_card.clone(),
        caterpie_hp,
        caterpie_hp,
        vec![EnergyType::Colorless],
        false,
        vec![],
    );
    state.in_play_pokemon[0][0] = Some(active);
    state
}

#[test]
fn test_caterpie_attack_retrieves_grass_pokemon() {
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let ivysaur = get_card_by_enum(CardId::A1002Ivysaur);
    let charmander = get_card_by_enum(CardId::A1033Charmander);
    let caterpie = get_card_by_enum(CardId::A1005Caterpie);

    let mut deck_a = Deck::default();
    deck_a.cards = vec![bulbasaur.clone(), ivysaur.clone(), charmander.clone()];
    let deck_b = Deck::default();

    let hp = match &caterpie {
        Card::Pokemon(p) => p.hp,
        _ => 0,
    };

    let state = setup_state(&deck_a, &deck_b, hp, &caterpie);

    let players: Vec<Box<dyn Player>> = vec![
        Box::new(EndTurnPlayer { deck: deck_a.clone() }),
        Box::new(EndTurnPlayer { deck: deck_b.clone() }),
    ];
    let mut game = Game::from_state(state, players, 0);

    let action = Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    };
    game.apply_action(&action);

    let result_state = game.get_state_clone();
    assert_eq!(result_state.hands[0].len(), 1);
    let card_drawn = &result_state.hands[0][0];
    if let Card::Pokemon(p) = card_drawn {
        assert_eq!(p.energy_type, EnergyType::Grass);
    } else {
        panic!("Expected Pokémon card");
    }
    assert_eq!(result_state.decks[0].cards.len(), 2);
    let energy = &result_state.in_play_pokemon[0][0]
        .as_ref()
        .unwrap()
        .attached_energy;
    assert_eq!(energy, &vec![EnergyType::Colorless]);
}

#[test]
fn test_caterpie_attack_probabilities() {
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let ivysaur = get_card_by_enum(CardId::A1002Ivysaur);
    let charmander = get_card_by_enum(CardId::A1033Charmander);
    let caterpie = get_card_by_enum(CardId::A1005Caterpie);

    let mut deck_a = Deck::default();
    deck_a.cards = vec![bulbasaur.clone(), ivysaur.clone(), charmander.clone()];
    let deck_b = Deck::default();

    let hp = match &caterpie {
        Card::Pokemon(p) => p.hp,
        _ => 0,
    };

    let iterations = 200;
    let mut counts: HashMap<String, usize> = HashMap::new();

    for i in 0..iterations {
        let state = setup_state(&deck_a, &deck_b, hp, &caterpie);
        let players: Vec<Box<dyn Player>> = vec![
            Box::new(EndTurnPlayer { deck: deck_a.clone() }),
            Box::new(EndTurnPlayer { deck: deck_b.clone() }),
        ];
        let mut game = Game::from_state(state, players, i as u64);
        let action = Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        };
        game.apply_action(&action);
        let card = game.get_state_clone().hands[0][0].get_name();
        *counts.entry(card).or_default() += 1;
    }

    // Should never draw the non-Grass card
    assert!(counts.get(&charmander.get_name()).unwrap_or(&0) == &0);

    let bulba = *counts.get(&bulbasaur.get_name()).unwrap_or(&0) as f64;
    let ivy = *counts.get(&ivysaur.get_name()).unwrap_or(&0) as f64;
    let total = bulba + ivy;
    assert_eq!(total as usize, iterations);

    let prob_bulba = bulba / total;
    let prob_ivy = ivy / total;
    // Both probabilities should be roughly equal
    assert!((prob_bulba - 0.5).abs() < 0.2);
    assert!((prob_ivy - 0.5).abs() < 0.2);
}
