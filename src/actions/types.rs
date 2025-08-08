use crate::{
    tool_ids::ToolId,
    types::{Card, EnergyType, TrainerCard},
};
use std::fmt;

/// Main structure for following Game Tree design. Using "nesting" with a
/// SimpleAction to share common fields here.
#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub actor: usize,
    pub action: SimpleAction,
    pub is_stack: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimpleAction {
    DrawCard {
        amount: u8,
    },
    Play {
        trainer_card: TrainerCard,
    },

    // Card because of the fossil Trainer Cards...
    // usize is bench 1-based index, with 0 meaning Active pokemon, 1..4 meaning Bench
    Place(Card, usize),
    Evolve(Card, usize),
    UseAbility(usize),

    // Its given it is with the active pokemon, to the other active.
    // usize is the index of the attack in the pokemon's attacks
    Attack(usize),
    // usize is in_play_pokemon index to retreat to. Can't Retreat(0)
    Retreat(usize),
    EndTurn,

    // Atomic actions as part of different effects.
    Attach {
        attachments: Vec<(u32, EnergyType, usize)>, // (amount, energy_type, in_play_idx)
        is_turn_energy: bool, // true if this is the energy from the zone that can be once per turn
    },
    AttachTool {
        in_play_idx: usize,
        tool_id: ToolId,
    },
    Heal {
        in_play_idx: usize,
        amount: u32,
    },
    ApplyDamage {
        targets: Vec<(u32, usize)>, // Vec of (damage, in_play_idx)
    },
    /// Switch the in_play_idx pokemon with the active pokemon.
    Activate {
        in_play_idx: usize,
    },
    /// Force the opponent to switch the specified bench Pokémon with their active Pokémon.
    ForceSwitchOpponent {
        in_play_idx: usize,
    },
    Noop, // No operation, used to have the user say "no" to a question
}

impl fmt::Display for SimpleAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimpleAction::DrawCard { amount } => write!(f, "DrawCard({amount})"),
            SimpleAction::Play { trainer_card } => write!(f, "Play({trainer_card:?})"),
            SimpleAction::Place(card, index) => write!(f, "Place({card}, {index})"),
            SimpleAction::Evolve(card, index) => write!(f, "Evolve({card}, {index})"),
            SimpleAction::UseAbility(index) => write!(f, "UseAbility({index})"),
            SimpleAction::Attack(index) => write!(f, "Attack({index})"),
            SimpleAction::Retreat(index) => write!(f, "Retreat({index})"),
            SimpleAction::EndTurn => write!(f, "EndTurn"),
            SimpleAction::Attach {
                attachments,
                is_turn_energy,
            } => {
                let attachments_str = attachments
                    .iter()
                    .map(|(amount, energy_type, in_play_idx)| {
                        format!("({amount}, {energy_type:?}, {in_play_idx})")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "Attach({attachments_str:?}, {is_turn_energy})")
            }
            SimpleAction::AttachTool {
                in_play_idx,
                tool_id,
            } => {
                write!(f, "AttachTool({in_play_idx}, {tool_id:?})")
            }
            SimpleAction::Heal {
                in_play_idx,
                amount,
            } => write!(f, "Heal({in_play_idx}, {amount})"),
            SimpleAction::ApplyDamage { targets } => {
                let targets_str = targets
                    .iter()
                    .map(|(damage, in_play_idx)| format!("({damage}, {in_play_idx})"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "ApplyDamage({targets_str})")
            }
            SimpleAction::Activate { in_play_idx } => write!(f, "Activate({in_play_idx})"),
            SimpleAction::ForceSwitchOpponent { in_play_idx } => {
                write!(f, "ForceSwitchOpponent({in_play_idx})")
            }
            SimpleAction::Noop => write!(f, "Noop"),
        }
    }
}
