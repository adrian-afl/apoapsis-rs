use crate::main_menu_stage::MainMenuStage;
use crate::splash_screen_stage::SplashScreenStage;
use core::game::Game;
use core::game_stage_trait::GameStage;
use std::sync::Arc;

pub struct StageFactory {
    game: Arc<Game>,
}

enum GameStagesEnum {
    SplashScreen,
    MainMenu,
    Gaming,
}

impl StageFactory {
    pub fn new(game: Arc<Game>) -> Self {
        Self { game }
    }

    pub fn create_stage(&self, stage: GameStagesEnum) -> Box<dyn GameStage> {
        let context = self.game.get_context();
        match stage {
            GameStagesEnum::SplashScreen => Box::new(SplashScreenStage::new()),
            GameStagesEnum::MainMenu => Box::new(MainMenuStage::new(&context)),
            GameStagesEnum::Gaming => panic!("Not implemented"),
        }
    }
}
