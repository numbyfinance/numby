use std::env;

use anymap3::AnyMap;
use cucumber::World;
use thirtyfour::{WebDriver, prelude::*};

#[derive(Clone, Debug)]
pub struct Config {
    pub webdriver_url: String,
    pub host: String,
    pub headless: bool,
}

impl Config {
    pub fn new() -> Config {
        let webdriver_url: String = if env::var("WEB_DRIVER_URL").is_ok() {
            env::var("WEB_DRIVER_URL").unwrap()
        } else {
            "http://localhost:4444".into()
        };

        let headless = env::var("WEB_DRIVER_HEADLESS").is_ok();

        let host = if env::var("WEB_DRIVER_DESTINATION_HOST").is_ok() {
            env::var("WEB_DRIVER_DESTINATION_HOST").unwrap()
        } else {
            "http://host.docker.internal:3000".into()
        };

        Config {
            webdriver_url,
            host,
            headless,
        }
    }

    pub async fn get_driver(&self) -> WebDriverResult<WebDriver> {
        let mut caps = DesiredCapabilities::chrome();
        caps.add_arg("--no-sandbox")?;
        caps.add_arg("--disable-gpu")?;
        caps.add_arg("--start-maximized")?;

        if self.headless {
            caps.set_headless()?;
        }

        WebDriver::new(&self.webdriver_url, caps).await
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

pub mod prelude {
    use cucumber::{given, when};

    use super::*;

    #[derive(Debug, Default, World)]
    pub struct CommonWorld {
        pub config: Config,
        pub driver: Option<WebDriver>,
        pub state: AnyMap,
    }

    impl CommonWorld {
        pub fn set_state<T: 'static>(&mut self, state: T) {
            self.state.insert(state);
        }

        pub fn state<T: 'static>(&self) -> Option<&T> {
            self.state.get::<T>()
        }

        pub fn state_mut<T: 'static>(&mut self) -> Option<&mut T> {
            self.state.get_mut::<T>()
        }
    }

    #[given("a webdriver")]
    async fn webdriver(world: &mut CommonWorld) {
        world.driver = Some(
            world
                .config
                .get_driver()
                .await
                .expect("Couldn't get webdriver"),
        );
    }

    #[when(regex = r"^we go to /(.+)$")]
    async fn navigate_to_path(world: &mut CommonWorld, path: String) {
        let driver = &world.driver.as_ref().expect("no webdriver");
        let url = format!("{}/{}", world.config.host, path);
        driver
            .goto(&url)
            .await
            .expect(&format!("Failed to navigate to /{}", path));
    }
}
