pub mod ai_config;
pub mod game_config;
pub mod game_map_config;
pub mod game_obj_config;
pub mod weapon_config;

pub use ai_config::{AiConfig, ChaseShootAiConfig};
pub use game_config::GameConfig;
pub use game_map_config::{GameMapConfig, GameMapObjConfig};
pub use game_obj_config::{
    BotConfig, DamageConfig, EnemySearchConfig, GameObjConfig, GameObjSide, GoodieConfig,
    GoodieEffect, MissileConfig, MissileFeature, NamedGameObjConfig, OnDeathAction, PierceConfig,
    PlayFrameConfig, SpawnMissileConfig, TileConfig,
};
pub use weapon_config::{GunComponentConfig, GunConfig, WeaponConfig};
