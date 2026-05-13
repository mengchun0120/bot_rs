pub mod create_obj;
pub mod game_obj;
pub mod move_obj;
pub mod on_death_action;
pub mod playout;
pub mod shoot;

pub use create_obj::{create_obj_by_config, create_obj_by_index};
pub use game_obj::{GameObj, GameObjState, GameObjType, MapPos};
pub use move_obj::{MoveResult, move_bot, move_missile, update_obj_pos};
pub use on_death_action::on_death;
pub use playout::{Phaseout, PlayFrame, Playout};
pub use shoot::try_shoot;

pub mod components {
    pub mod ai_comp;
    pub mod markers;
    pub mod missile_comp;
    pub mod playout_comp;
    pub mod weapon_comp;

    pub use ai_comp::AiComponent;
    pub use markers::{AiBotComponent, InView, PlayerComponent, TileComponent};
    pub use missile_comp::{EnemySearchAbility, MissileComponent, PierceAbility};
    pub use playout_comp::PlayoutComponent;
    pub use weapon_comp::WeaponComponent;
}
