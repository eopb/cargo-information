use cargo_test_macro::cargo_test;
use cargo_test_support::{compare::assert_ui, current_dir, file, Project};

use super::{cargo_info, init_registry_without_token};

#[cargo_test]
fn case() {
    init_registry_without_token();
    for ver in [
        "0.1.1+my-package",
        "0.2.0+my-package",
        "0.2.3+my-package",
        "0.4.1+my-package",
        "20.0.0+my-package",
        "99999.0.0+my-package",
        "99999.0.0-alpha.1+my-package",
    ] {
        cargo_test_support::registry::Package::new("my-package", ver).publish();
    }

    let project = Project::from_template(current_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root;

    cargo_info()
        .arg("my-package")
        .arg("--registry=dummy-registry")
        .current_dir(cwd)
        .assert()
        .success()
        .stdout_matches(file!["stdout.log"])
        .stderr_matches(file!["stderr.log"]);

    assert_ui().subset_matches(current_dir!().join("out"), &project_root);
}
