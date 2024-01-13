mod modules;

#[tokio::main]
async fn main() {
    start().await;

    let a = modules::loader::injector::inject("cs2.exe", "https://cdn.discordapp.com/attachments/1190304279236464711/1195104396875403384/Osiris.dll?ex=65b2c6b4&is=65a051b4&hm=68edd1962711cdd8f90f22c1d2cfeafccf5b0c2ff299e45c9cd7d526ac40f07d&").await;

    println!("{:?}", a);
}

async fn start() {
    std::fs::create_dir_all(format!(
        "{}{}",
        modules::loader::settings::PATH,
        "repositories"
    ))
    .unwrap();
    modules::loader::settings::create().await;
}
