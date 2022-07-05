#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    LoadingAssets,
    Setup,
    Running,
}
