pub mod settings {
    use serde_json;

    pub const PATH: &str = "C:\\MLoader\\";
    const SETTINGS_PATH: &str = "settings.json";

    pub async fn create() {
        if std::fs::metadata(format!("{}{}", PATH, SETTINGS_PATH)).is_err() {
            std::fs::File::create(format!("{}{}", PATH, SETTINGS_PATH)).unwrap();

            std::fs::write(
                format!("{}{}", PATH, SETTINGS_PATH),
                serde_json::json!({
                    "username": "",
                    "repositories": [

                    ],
                })
                .to_string(),
            )
            .unwrap();
        }
    }

    pub async fn get(key: &str) -> serde_json::Value {
        let settings: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(format!("{}{}", PATH, SETTINGS_PATH)).unwrap(),
        )
        .unwrap();

        settings[key].clone()
    }

    pub async fn replace<T: serde::ser::Serialize>(key: &str, value: T) {
        let mut settings: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(format!("{}{}", PATH, SETTINGS_PATH)).unwrap(),
        )
        .unwrap();

        match settings.get_mut(&key) {
            Some(v) => *v = serde_json::json!(value),
            None => {}
        }

        std::fs::write(
            format!("{}{}", PATH, SETTINGS_PATH),
            serde_json::to_string_pretty(&settings).unwrap(),
        )
        .unwrap();
    }
}

pub mod injector {
    use std::os::windows::ffi::OsStrExt;

    use libloading;
    use serde_json;

    use super::settings;

    pub async fn inject(name: &str, url: &str) -> serde_json::Value {
        unsafe {
            match libloading::Library::new(format!("{}injector.dll", settings::PATH)) {
                Ok(lib) => {
                    let inject_func: libloading::Symbol<
                        unsafe extern "C" fn(*const u16, *const i8) -> bool,
                    > = lib.get(b"inject").expect("Failed to get function");

                    let name = std::ffi::OsStr::new(name)
                        .encode_wide()
                        .chain(std::iter::once(0))
                        .collect::<Vec<_>>();

                    let url =
                        std::ffi::CString::new(url).expect("Failed to convert URL to CString");

                    let result = inject_func(name.as_ptr(), url.as_ptr().cast());

                    match result {
                        true => serde_json::json!({
                            "status": "success",
                            "message": "Injection successful!",
                        }),
                        false => serde_json::json!({
                            "status": "error",
                            "message": "Injection failed!",
                        }),
                    }
                }
                Err(error) => {
                    serde_json::json!({
                        "status": "error",
                        "message": error.to_string(),
                    })
                }
            }
        }
    }
}
