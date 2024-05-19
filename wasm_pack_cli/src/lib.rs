const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Run wasm-pack with the given arguments.
///
/// ```
/// let args = vec![
///     "build",
///     "--out-dir",
///     // If we just passed "target/built-test-crate",
///     // the output would be in "./test-crate/target/built-test-crate".
///     // So instead, we go up a directory.
///     "../target/built-test-crate",
///     // The input crate path is relative to the current directory.
///     "test-crate",
/// ];
///
/// wasm_pack_cli::run(args).expect("Running wasm-pack failed.");
/// ```
pub fn run(args: Vec<&str>) -> Result<run_binary::Output, run_binary::RunBinaryError> {
    println!("Running wasm-pack with args: {:?}", args);

    let bytes = get_cli_executable_bytes();

    run_binary::run_binary(run_binary::RunBinaryParams {
        binary_name: "wasm-pack",
        version: CRATE_VERSION,
        bytes,
        arguments_for_binary: args,
    })
}

fn get_cli_executable_bytes() -> &'static [u8] {
    let platform = guess_platform();
    println!("Guessed platform: {:?}", platform);
    match platform {
        Platform::MacOs => include_bytes!("./wasm-pack-v0.12.1-x86_64-apple-darwin/wasm-pack"),
        Platform::LinuxArm64 => {
            include_bytes!("./wasm-pack-v0.12.1-aarch64-unknown-linux-musl/wasm-pack")
        }
        Platform::LinuxX64 => {
            include_bytes!("./wasm-pack-v0.12.1-x86_64-unknown-linux-musl/wasm-pack")
        }
        Platform::Windows => {
            include_bytes!("./wasm-pack-v0.12.1-x86_64-pc-windows-msvc/wasm-pack.exe")
        }
    }
}

fn guess_platform() -> Platform {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    match os {
        "macos" => Platform::MacOs,
        "linux" => match arch {
            "x86_64" => Platform::LinuxX64,
            "aarch64" => Platform::LinuxArm64,
            _ => panic!("Unsupported Linux architecture: {}", arch),
        },
        "windows" => Platform::Windows,
        _ => panic!("Unsupported OS: {}", os),
    }
}

#[derive(Debug)]
enum Platform {
    MacOs,

    LinuxArm64,
    LinuxX64,

    Windows,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Platform::MacOs => "x86_64-apple-darwin",
            Platform::LinuxArm64 => "aarch64-unknown-linux-musl",
            Platform::LinuxX64 => "x86_64-unknown-linux-musl",
            Platform::Windows => "x86_64-pc-windows-msvc",
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use run_binary::RunBinaryError;

    #[test]
    fn version_is_correct() {
        let args = vec!["--version"];
        let output = run(args).expect("Couldn't run `wasm-pack --version`.");
        let stdout = output.stdout();

        // Our crate versions are a string like "0.12.1-0.1.0". Everything before
        // the dash is the wasm-pack version. See the version policy in the README
        // for details.
        let expected_version = CRATE_VERSION.split('-').next().unwrap();
        let expected_version = format!("wasm-pack {}", expected_version);
        println!("Command stdout: {}", &stdout);
        println!("Expected version: {}", &expected_version);
        assert!(stdout.contains(&expected_version));
    }

    #[test]
    fn building_a_crate() {
        let input_crate_path = "test-crate";
        let built_crate_path = "target/built-test-crate";
        let built_crate_path_relative_to_input_crate = "../".to_string() + built_crate_path; // This is relative to the input crate path, so we have to go up a directory.

        let _ignore_errors = std::fs::remove_dir_all(built_crate_path);

        let args = vec![
            "build",
            "--out-dir",
            &built_crate_path_relative_to_input_crate,
            input_crate_path,
        ];
        run(args).expect("Couldn't run `wasm-pack`.");

        let built_js = std::fs::read_to_string(format!("{}/test_crate.js", built_crate_path))
            .expect("Couldn't read built JS file.");

        let expected_built_js = r#"import * as wasm from "./test_crate_bg.wasm";
import { __wbg_set_wasm } from "./test_crate_bg.js";
__wbg_set_wasm(wasm);
export * from "./test_crate_bg.js";
"#;

        assert_eq!(built_js, expected_built_js);
    }

    #[test]
    fn input_file_not_found() {
        let input_crate_path = "fake-crate";
        let built_crate_path = "target/built-test-crate";
        let built_crate_path_relative_to_input_crate = "../".to_string() + built_crate_path; // This is relative to the input crate path, so we have to go up a directory.

        let args = vec![
            "build",
            "--out-dir",
            &built_crate_path_relative_to_input_crate,
            input_crate_path,
        ];

        let result = run(args);

        if let Err(RunBinaryError::BinaryReturnedAnError { stdout, stderr }) = result {
            assert!(stdout.is_empty());
            assert!(stderr.contains("Error: crate directory is missing a `Cargo.toml` file; is `fake-crate` the wrong directory?"));
        } else {
            panic!(
                "Expected RunBinaryError::BinaryReturnedAnError, got {:?}",
                result
            );
        }
    }
}
