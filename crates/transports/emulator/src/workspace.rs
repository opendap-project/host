//! Locating the emulator that runs virtual devices (`bin/odap-vdev`): the `emulator/` folder of the
//! `blackbird` root (RFC 0017), under `$ODAP_BLACKBIRD_DIR`, else under the workspace's `blackbird`
//! row in `repos.tsv` above the current directory (RFC 0008). The old `virtual-devices` variable and
//! row are not read any more (HL-91). Never a guessed relative path.

use std::path::{Path, PathBuf};

use opendap_host_model::Error;
use opendap_host_model::error as codes;

/// Environment variable that points at the blackbird checkout.
pub const VD_DIR_VAR: &str = "ODAP_BLACKBIRD_DIR";

/// The root the emulator is part of, and its folder there (RFC 0017).
pub const VD_ROOT: (&str, &str) = ("blackbird", "emulator");

/// A directory is an emulator when it has the tool.
pub fn is_vd_checkout(dir: &Path) -> bool {
    dir.join("bin/odap-vdev").is_file()
}

/// The `emulator/` folder of `$ODAP_BLACKBIRD_DIR` when set (it must have the tool), otherwise of the
/// workspace's `blackbird` row.
pub fn find_vd_checkout(env_value: Option<&str>, cwd: &Path) -> Result<PathBuf, Error> {
    if let Some(v) = env_value.filter(|v| !v.is_empty()) {
        let p = PathBuf::from(v);
        let p = if p.is_relative() { cwd.join(p) } else { p }.join(VD_ROOT.1);
        return if is_vd_checkout(&p) {
            Ok(p.canonicalize().unwrap_or(p))
        } else {
            Err(Error::new(
                &codes::WORKSPACE_NOT_FOUND,
                format!("{VD_DIR_VAR}={v} has no emulator (no emulator/bin/odap-vdev)"),
            )
            .hint(format!(
                "Point {VD_DIR_VAR} at the blackbird repository, or unset it."
            )))
        };
    }
    for dir in cwd.ancestors() {
        let Ok(tsv) = std::fs::read_to_string(dir.join("repos.tsv")) else {
            continue;
        };
        let found = tsv.lines().skip(1).find_map(|line| {
            let (path, slug) = line.split_once('\t')?;
            (slug.trim().rsplit('/').next() == Some(VD_ROOT.0))
                .then(|| dir.join(path).join(VD_ROOT.1))
        });
        return match found {
            Some(p) if is_vd_checkout(&p) => Ok(p.canonicalize().unwrap_or(p)),
            _ => Err(Error::new(
                &codes::WORKSPACE_NOT_FOUND,
                format!("{} is not checked out in {}", VD_ROOT.0, dir.display()),
            )
            .fix("scripts/bootstrap.sh")),
        };
    }
    Err(Error::new(
        &codes::WORKSPACE_NOT_FOUND,
        format!("no OpenDAP workspace (repos.tsv) above {}", cwd.display()),
    )
    .hint(format!(
        "Run from inside the workspace, or set {VD_DIR_VAR}."
    )))
}

/// Same as [`find_vd_checkout`], from the process environment and directory.
pub fn vd_checkout() -> Result<PathBuf, Error> {
    let cwd = std::env::current_dir().map_err(|e| {
        Error::new(
            &codes::HOST_IO,
            format!("cannot read the current directory: {e}"),
        )
    })?;
    find_vd_checkout(std::env::var(VD_DIR_VAR).ok().as_deref(), &cwd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_the_old_virtual_devices_row_and_takes_the_variable_s_emulator() {
        let root = std::env::temp_dir().join(format!("opendap-ws-{}", std::process::id()));
        std::fs::create_dir_all(root.join("public/virtual-devices/bin")).unwrap();
        std::fs::create_dir_all(root.join("elsewhere/emulator/bin")).unwrap();
        std::fs::create_dir_all(root.join("public/cli/x")).unwrap();
        std::fs::write(root.join("public/virtual-devices/bin/odap-vdev"), "").unwrap();
        std::fs::write(root.join("elsewhere/emulator/bin/odap-vdev"), "").unwrap();
        std::fs::write(
            root.join("repos.tsv"),
            "path\trepository\npublic/virtual-devices\topendap-project/virtual-devices\n",
        )
        .unwrap();
        assert!(find_vd_checkout(None, &root.join("public/cli/x")).is_err());
        let via_var = root.join("elsewhere");
        let found = find_vd_checkout(via_var.to_str(), &root).unwrap();
        assert!(found.ends_with("elsewhere/emulator"));
        assert!(find_vd_checkout(Some("/nonexistent"), &root).is_err());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn finds_the_emulator_folder_of_the_blackbird_root() {
        let root = std::env::temp_dir().join(format!("opendap-ws-bb-{}", std::process::id()));
        std::fs::create_dir_all(root.join("public/blackbird/emulator/bin")).unwrap();
        std::fs::create_dir_all(root.join("public/host/x")).unwrap();
        std::fs::write(root.join("public/blackbird/emulator/bin/odap-vdev"), "").unwrap();
        std::fs::write(
            root.join("repos.tsv"),
            "path\trepository\npublic/blackbird\topendap-project/blackbird\n",
        )
        .unwrap();
        let found = find_vd_checkout(None, &root.join("public/host/x")).unwrap();
        assert!(found.ends_with("public/blackbird/emulator"));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn reports_a_missing_workspace() {
        let e = find_vd_checkout(None, Path::new("/")).unwrap_err();
        assert!(e.message.contains("repos.tsv"));
    }
}
