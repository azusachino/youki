use std::path::{PathBuf, Path};
use std::process::{Command, Stdio};

// TODO Allow to receive arguments.
// TODO Wrapping up the results
pub fn exec(project_path: &Path, id: &str) -> bool {
    let status = Command::new(project_path.join(PathBuf::from("youki")))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-r")
        .arg(project_path.join("integration-workspace").join("youki"))
        .arg("start")
        .arg(id)
        .status()
        .expect("failed to execute process");
    status.success()
}
