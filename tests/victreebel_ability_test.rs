use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    generate_possible_actions,
    players::{EndTurnPlayer, Player},
    types::{Card, PlayedCard},
    Deck, Game, State,
};

#[test]
fn victreebel_fragrance_trap_switches_basic_from_opponent() {
    let victreebel = get_card_by_enum(CardId::A1020Victreebel);
    let charmander = get_card_by_enum(CardId::A1033Charmander);
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let ivysaur = get_card_by_enum(CardId::A1002Ivysaur);

    let mut state = State::default();
    state.turn_count = 1;
    let hp_vic = match &victreebel { Card::Pokemon(p) => p.hp, _ => 0 };
    state.in_play_pokemon[0][0] = Some(PlayedCard::new(victreebel.clone(), hp_vic, hp_vic, vec![], false, vec![]));

    let hp_char = match &charmander { Card::Pokemon(p) => p.hp, _ => 0 };
    state.in_play_pokemon[1][0] = Some(PlayedCard::new(charmander.clone(), hp_char, hp_char, vec![], false, vec![]));
    let hp_bulba = match &bulbasaur { Card::Pokemon(p) => p.hp, _ => 0 };
    state.in_play_pokemon[1][1] = Some(PlayedCard::new(bulbasaur.clone(), hp_bulba, hp_bulba, vec![], false, vec![]));
    let hp_ivy = match &ivysaur { Card::Pokemon(p) => p.hp, _ => 0 };
    state.in_play_pokemon[1][2] = Some(PlayedCard::new(ivysaur.clone(), hp_ivy, hp_ivy, vec![], false, vec![]));

    let players: Vec<Box<dyn Player>> = vec![
        Box::new(EndTurnPlayer { deck: Deck::default() }),
        Box::new(EndTurnPlayer { deck: Deck::default() }),
    ];
    let mut game = Game::from_state(state, players, 0);

    let action = Action { actor: 0, action: SimpleAction::UseAbility(0), is_stack: false };
    game.apply_action(&action);

    let state = game.get_state_clone();
    let (actor, actions) = generate_possible_actions(&state);
    assert_eq!(actor, 0);
    assert_eq!(actions.len(), 1);
    assert!(matches!(actions[0].action, SimpleAction::ForceSwitchOpponent { in_play_idx: 1 }));

    let switch_action = Action { actor: 0, action: SimpleAction::ForceSwitchOpponent { in_play_idx: 1 }, is_stack: true };
    game.apply_action(&switch_action);

    let state = game.get_state_clone();
    assert_eq!(state.in_play_pokemon[1][0].as_ref().unwrap().card.get_name(), bulbasaur.get_name());
    assert_eq!(state.in_play_pokemon[1][1].as_ref().unwrap().card.get_name(), charmander.get_name());
    assert_eq!(state.in_play_pokemon[1][2].as_ref().unwrap().card.get_name(), ivysaur.get_name());
}
