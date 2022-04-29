pub enum SaveGameState {
    NewGame,
    Save(String)
}

pub struct Save;
impl Save {
    pub fn check_for_saves() -> Result<bool, ()> {
        use std::fs;
        use std::path::Path;

        if Path::new("./saves").is_dir() {
            match fs::read_dir("./saves") {
                Ok(mut a) => {
                    if a.any(|_| true) {
                        Result::Ok(true)
                    } else {
                        Result::Ok(false)
                    }
                },
                Err(_) => Result::Err(())
            }
        } else {
            match fs::create_dir("./saves") {
                Ok(_) => Result::Ok(false),
                Err(_) => Result::Err(())
            }
        }
    }
}