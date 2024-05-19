const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Run the Tailwind CLI with the given arguments.
///
/// ```
/// let args = vec!["--input", "src/main.css", "--output", "target/built.css"];
/// tailwind_cli::run(args).expect("Running Tailwind CLI failed.");
/// ```
pub fn run(args: Vec<&str>) -> Result<run_binary::Output, run_binary::RunBinaryError> {
    println!("Running Tailwind CLI with args: {:?}", args);

    let bytes = get_cli_executable_bytes();

    run_binary::run_binary(run_binary::RunBinaryParams {
        binary_name: "tailwindcss",
        version: CRATE_VERSION,
        bytes,
        arguments_for_binary: args,
    })
}

#[derive(Debug)]
enum Platform {
    // macOS
    MacOsArm64,
    MacOsX64,

    // Linux
    LinuxArm64,
    LinuxArmv7,
    LinuxX64,

    // Windows
    WindowsArm64,
    WindowsX64,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Platform::MacOsArm64 => "macos-arm64",
            Platform::MacOsX64 => "macos-x64",
            Platform::LinuxArm64 => "linux-arm64",
            Platform::LinuxArmv7 => "linux-armv7",
            Platform::LinuxX64 => "linux-x64",
            Platform::WindowsArm64 => "windows-arm64",
            Platform::WindowsX64 => "windows-x64",
        };
        write!(f, "{}", name)
    }
}

fn guess_platform() -> Platform {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    match os {
        "macos" => match arch {
            "x86_64" => Platform::MacOsX64,
            "aarch64" => Platform::MacOsArm64,
            _ => panic!("Unsupported architecture: {}", arch),
        },
        "linux" => match arch {
            "x86_64" => Platform::LinuxX64,
            "aarch64" => Platform::LinuxArm64,
            "armv7" => Platform::LinuxArmv7,
            _ => panic!("Unsupported architecture: {}", arch),
        },
        "windows" => match arch {
            "x86_64" => Platform::WindowsX64,
            "aarch64" => Platform::WindowsArm64,
            _ => panic!("Unsupported architecture: {}", arch),
        },
        _ => panic!("Unsupported OS: {}", os),
    }
}

fn get_cli_executable_bytes() -> &'static [u8] {
    let platform = guess_platform();
    println!("Guessed platform: {:?}", platform);
    match platform {
        Platform::MacOsArm64 => include_bytes!("./tailwindcss-macos-arm64"),
        Platform::MacOsX64 => include_bytes!("./tailwindcss-macos-x64"),
        Platform::LinuxArm64 => include_bytes!("./tailwindcss-linux-arm64"),
        Platform::LinuxArmv7 => include_bytes!("./tailwindcss-linux-armv7"),
        Platform::LinuxX64 => include_bytes!("./tailwindcss-linux-x64"),
        Platform::WindowsArm64 => include_bytes!("./tailwindcss-windows-arm64.exe"),
        Platform::WindowsX64 => include_bytes!("./tailwindcss-windows-x64.exe"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_correct() {
        let args = vec!["--help"];
        let output = run(args).expect("Couldn't run `tailwindcss --help`.");
        let stdout = output.stdout();

        // Our crate versions are a string like "3.4.1-0". 3.4.1 is the Tailwind
        // version. 0 is the version of this crate, useful if we need to release
        // a new version of this crate without changing the Tailwind version.
        //
        // Strip everything after the first dash to get the Tailwind version.
        let expected_version = CRATE_VERSION.split('-').next().unwrap();
        let expected_version = format!("tailwindcss v{}", expected_version);
        println!("Command stdout: {}", &stdout);
        println!("Expected version: {}", &expected_version);
        assert!(stdout.contains(&expected_version));
    }

    #[test]
    fn built_css_has_expected_classes() {
        let built_css_path = "target/built.css";

        let _ignore_errors = std::fs::remove_file(built_css_path);

        let args = vec!["--input", "src/main.css", "--output", built_css_path];
        run(args).expect("Couldn't run `tailwindcss`.");

        let font_bold_declaration = ".font-bold {
  font-weight: 700;
}";

        let built_css =
            std::fs::read_to_string(built_css_path).expect("Couldn't read built CSS file.");

        assert!(built_css.contains(font_bold_declaration));
    }

    #[test]
    fn input_file_not_found() {
        let built_css_path = "target/built.css";

        let _ignore_errors = std::fs::remove_file(built_css_path);

        let args = vec![
            "--input",
            "src/doesnt_exist.css",
            "--output",
            built_css_path,
        ];
        let result = run(args);

        if let Err(run_binary::RunBinaryError::BinaryReturnedAnError { stdout, stderr }) = result {
            assert!(stdout.is_empty());
            assert!(stderr.contains("Specified input file src/doesnt_exist.css does not exist."));
        } else {
            panic!(
                "Expected TailwindCliReturnedAnError error, got {:?}",
                result
            );
        }
    }
}
