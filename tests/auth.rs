use cucumber::{World, then, when};
use thirtyfour::By;

mod utils;
use utils::prelude::*;

#[derive(Default, Debug)]
struct AuthState {}

#[then("we are logged in")]
async fn verify_logged_in(world: &mut CommonWorld) {
    let driver = &world.driver.as_ref().expect("no webdriver");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let current_url = driver
        .current_url()
        .await
        .expect("Failed to get current URL");
    assert!(current_url.as_str().contains("/tp"));
}

#[when("input correct credentials")]
async fn input_correct_credentials(world: &mut CommonWorld) {
    let driver = &world.driver.as_ref().expect("no webdriver");

    let email_field = driver
        .find(By::Id("email"))
        .await
        .expect("Could not find email field");
    email_field
        .send_keys("topaz@ipc.org")
        .await
        .expect("Could not input email");

    let password_field = driver
        .find(By::Id("password"))
        .await
        .expect("Could not find password field");
    password_field
        .send_keys("topaz")
        .await
        .expect("Could not input password");

    let sign_in_button = driver
        .find(By::Id("sign-in"))
        .await
        .expect("Could not find sign in button");
    sign_in_button
        .click()
        .await
        .expect("Could not click sign in button");
}

#[tokio::main]
async fn main() {
    let runner = CommonWorld::cucumber().before(|_feature, _rule, _scenario, world| {
        Box::pin(async move {
            if world.state::<AuthState>().is_none() {
                world.set_state(AuthState::default());
            }
        })
    });

    runner.run("tests/features/auth.feature").await;
}
