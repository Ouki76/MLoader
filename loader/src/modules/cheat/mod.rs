pub mod lua {
    use std::io::Read;

    use rlua::{self, ToLua, ToLuaMulti};

    use crate::modules::loader::injector;

    pub async fn run(path: &std::path::Path) {
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
