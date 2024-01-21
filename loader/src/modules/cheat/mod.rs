pub mod lua {
    use std::io::Read;

    use rlua::{self, ToLua, ToLuaMulti};

    use crate::modules::loader::injector;

    #[tauri::command]
    pub async fn run_script(path: String) {
        let path = std::path::Path::new(path.as_str());

        let lua = rlua::Lua::new();

        let mut file = std::fs::File::open(path).unwrap();
        let mut script = String::new();

        file.read_to_string(&mut script).unwrap();

        lua.context(|ctx| {
            let inj_table = ctx.create_table().unwrap();

            inj_table
                .set(
                    "inject",
                    ctx.create_function(|_string, (name, url): (String, String)| {
                        Ok(injector::inject(&name, &url).to_string())
                    })
                    .unwrap(),
                )
                .unwrap();

            inj_table
                .set(
                    "getModule",
                    ctx.create_function(|_string, (proc_name, name): (String, String)| {
                        Ok(injector::get_module(&proc_name, &name).to_string())
                    })
                    .unwrap(),
                )
                .unwrap();

            inj_table
                .set(
                    "getPid",
                    ctx.create_function(|_string, name: String| {
                        Ok(injector::get_pid(&name).to_string())
                    })
                    .unwrap(),
                )
                .unwrap();

            ctx.globals().set("Injector", inj_table).unwrap();

            let utils_table = ctx.create_table().unwrap();

            utils_table
                .set(
                    "Path",
                    path.parent()
                        .map(|parent| parent.to_path_buf())
                        .unwrap_or_default()
                        .to_str()
                        .unwrap(),
                )
                .unwrap();

            ctx.globals().set("Utils", utils_table).unwrap();

            let json_library = ctx.load(include_str!("json.lua")).into_function().unwrap();

            ctx.globals().set("json", json_library).unwrap();

            ctx.load("json = json()").exec().unwrap();

            ctx.load(&script).exec().unwrap();
        });
    }
}

pub mod parser {

    use crate::modules::loader::settings;
    use walkdir::WalkDir;

    use git2;

    pub async fn get_active_cheats() -> Vec<String> {
        let mut cheats = Vec::<String>::new();

        for entry in WalkDir::new(format!("{}repositories", settings::PATH))
            .into_iter()
            .filter(|entry| entry.is_ok())
            .map(|entry| entry.unwrap())
        {
            if entry.file_name() == "settings.json" {
                cheats.push(entry.path().display().to_string());
            }
        }

        cheats
    }

    #[tauri::command]
    pub async fn get_cheats_json() -> String {
        let cheats = get_active_cheats();

        let jsons = cheats
            .await
            .iter()
            .map(|repo| {
                let mut repo_json: serde_json::Value =
                    serde_json::from_str(&std::fs::read_to_string(repo).unwrap()).unwrap();

                let path = std::path::Path::new(repo.as_str())
                    .parent()
                    .clone()
                    .map(|parent| parent.to_path_buf())
                    .unwrap_or_default();

                repo_json["path"] = serde_json::Value::String(path.to_str().unwrap().to_string());

                repo_json["type"] = serde_json::Value::String(match git2::Repository::open(path) {
                    Ok(_) => "git".to_string(),
                    Err(_) => "local".to_string(),
                });

                repo_json
            })
            .collect::<Vec<serde_json::Value>>();

        serde_json::to_string(&jsons).unwrap()
    }
}
