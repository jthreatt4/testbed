use crate::prelude::*;

#[derive(Resource)]
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct AssetList(pub Vec<HandleUntyped>);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TurnState {
    #[default]
    LoadAssets,
    StartScreen,
    AwaitingInput,
    InMenus,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
}
