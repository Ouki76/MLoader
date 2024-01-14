use std::io::Write;

mod modules;

#[tokio::main]
async fn main() {
    start().await;

    modules::cheat::lua::run(std::path::Path::new(
        "C:\\MLoader\\repositories\\settings.lua",
    ))
    .await;
}

async fn start() {
    std::fs::create_dir_all(format!(
        "{}{}",
        modules::loader::settings::PATH,
        "repositories"
    ))
    .unwrap();

    modules::loader::settings::create().await;

    std::fs::File::create(format!("{}injector.dll", modules::loader::settings::PATH))
        .unwrap()
        .write(include_bytes!("../../BlackBone/build/Release/injector.dll"))
        .unwrap();
}
