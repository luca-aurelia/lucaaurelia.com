use std::fmt::Display;
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub struct RunBinaryParams<'binary_args> {
    /// The name of the file. For example, "tailwindcss".
    pub binary_name: &'static str,
    /// What version of the software is this? For example, "v3.4.1".
    pub version: &'static str,
    /// The actual bytes of the executable file.
    pub bytes: &'static [u8],
    /// The arguments to pass to the binary when we run it.
    pub arguments_for_binary: Vec<&'binary_args str>,
}

/// Save the given binary bytes to a temporary file and run it.
///
/// ```
/// run_binary::run_binary(RunBinaryParams {
///     binary_name: "tailwindcss",
///     version: "v3.4.1",
///     bytes: include_bytes!("./tailwindcss-v3.4.1"),
///     arguments_for_binary: vec!["--input", "src/main.css", "--output", "target/built.css"],
/// });
/// ```
pub fn run_binary(params: RunBinaryParams) -> Result<Output, RunBinaryError> {
    println!(
        "Running binary {:?}-{:?} with args: {:?}",
        params.binary_name, params.version, params.arguments_for_binary
    );

    let path_to_cli_executable = get_path_to_executable_file(GetPathToExecutableFileParams {
        binary_name: params.binary_name,
        version: params.version,
        bytes: params.bytes,
    })?;

    println!(
        "Got path to executable file for binary: {:?}",
        path_to_cli_executable
    );
    println!("Running binary...");
    let output = duct::cmd(&path_to_cli_executable, params.arguments_for_binary)
        .stderr_capture()
        .stdout_capture()
        .unchecked()
        .run()
        .map_err(RunBinaryError::CouldntRunBinary)?;

    let (stdout, stderr) = get_stdout_and_stderr_from_process_output(&output);

    println!("Binary finished running.");
    println!("Binary stdout: {}", &stdout);
    println!("Binary stderr: {}", &stderr);

    if !output.status.success() {
        println!("Binary returned an error.");
        let error = RunBinaryError::BinaryReturnedAnError { stdout, stderr };
        return Err(error);
    }

    println!("Binary returned successfully.");
    let output = Output::new(output);
    Ok(output)
}

#[derive(Debug)]
pub struct Output {
    stdout: String,
    stderr: String,
}

impl Output {
    fn new(process_output: std::process::Output) -> Self {
        let (stdout, stderr) = get_stdout_and_stderr_from_process_output(&process_output);
        Self { stdout, stderr }
    }

    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    pub fn stderr(&self) -> &str {
        &self.stderr
    }
}

fn get_stdout_and_stderr_from_process_output(
    process_output: &std::process::Output,
) -> (String, String) {
    let stdout = String::from_utf8_lossy(&process_output.stdout)
        .trim()
        .to_string();

    let stderr = String::from_utf8_lossy(&process_output.stderr)
        .trim()
        .to_string();

    (stdout, stderr)
}

pub struct GetPathToExecutableFileParams {
    /// The name of the file. For example, "tailwindcss".
    binary_name: &'static str,
    /// What version of the software is this? For example, "v3.4.1".
    version: &'static str,
    /// The actual bytes of the executable file.
    bytes: &'static [u8],
}

// This function has some intentional limitations. For example, if saving the file
// works but something goes wrong after that, we don't clean up the file. This means
// further invocations will fail because the file already exists, but in a broken state.
fn get_path_to_executable_file(
    params: GetPathToExecutableFileParams,
) -> Result<PathBuf, RunBinaryError> {
    // We use a UUID in case multiple builds are running at the same time.
    // let uuid = uuid::Uuid::new_v4().to_string();
    let temp_file_name = format!("{}-{}", params.binary_name, params.version /*, uuid */);
    let temp_file_path = std::env::current_dir()
        .map_err(RunBinaryError::CouldntSaveBytesToTemporaryFile)?
        .join("target")
        .join(temp_file_name);

    println!("Temporary file path: {:?}", &temp_file_path);

    // If we've already written the file to disk, return the path.
    if temp_file_path.exists() {
        return Ok(temp_file_path);
    }

    // Make sure all the directories in the path exist.
    std::fs::create_dir_all(temp_file_path.parent().unwrap())
        .map_err(RunBinaryError::CouldntSaveBytesToTemporaryFile)?;

    let mut temp_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&temp_file_path)
        .map_err(RunBinaryError::CouldntSaveBytesToTemporaryFile)?;

    println!("Created temporary file: {:?}", &temp_file_path);

    temp_file
        .write_all(params.bytes)
        .map_err(RunBinaryError::CouldntSaveBytesToTemporaryFile)?;
    println!("Wrote CLI executable bytes to temporary file.");

    // Make the file executable. This isn't supported on Windows, so we skip it.
    #[cfg(unix)]
    {
        println!("Setting permissions.");
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = temp_file
            .metadata()
            .map_err(RunBinaryError::CouldntSaveBytesToTemporaryFile)?
            .permissions();
        // 755 - owner can read/write/execute, group/others can read/execute.
        permissions.set_mode(0o755);
        temp_file
            .set_permissions(permissions)
            .map_err(RunBinaryError::CouldntSaveBytesToTemporaryFile)?;
        println!("Made temporary file executable.");
    }

    // Make sure the file is closed and written to disk.
    temp_file
        .sync_all()
        .map_err(RunBinaryError::CouldntSaveBytesToTemporaryFile)?;
    drop(temp_file);

    Ok(temp_file_path)
}

#[derive(Debug)]
pub enum RunBinaryError {
    BinaryReturnedAnError { stdout: String, stderr: String },
    CouldntRunBinary(io::Error),
    CouldntSaveBytesToTemporaryFile(io::Error),
}

impl Display for RunBinaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunBinaryError::BinaryReturnedAnError { stdout, stderr } => {
                write!(f, "Binary returned an error:\n\n")?;
                write!(f, "stdout:\n{}\n\n", stdout)?;
                write!(f, "stderr:\n{}\n\n", stderr)?;
                Ok(())
            }
            RunBinaryError::CouldntRunBinary(error) => {
                write!(f, "Couldn't run the binary: {}", error)
            }
            RunBinaryError::CouldntSaveBytesToTemporaryFile(error) => {
                write!(
                    f,
                    "Couldn't save binary bytes to a temporary file: {}",
                    error
                )
            }
        }
    }
}

impl std::error::Error for RunBinaryError {}
