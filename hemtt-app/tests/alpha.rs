use std::fs::File;

use hemtt::Project;

use semver::Version;

#[test]
fn project_info() {
    std::env::set_current_dir("./tests/mod_alpha").unwrap();
    let root = std::env::current_dir().unwrap();
    let project = Project::read().unwrap();
    assert_eq!(project.name(), "CBA Base Template");
    assert_eq!(project.author(), "CBA Base Template");
    assert_eq!(project.prefix(), "test");
    assert_eq!(project.mainprefix(), "z");
    assert_eq!(project.template(), "cba");
    assert_eq!(project.version(), &Version::new(0, 1, 0));
    assert_eq!(project.authority().unwrap(), "cba_base_template-0.1.0");
    assert_eq!(project.sig_version(), 3);

    assert_eq!(hemtt::project::get_all_addons().unwrap().len(), 1);

    assert_eq!(Project::find_root().unwrap(), root);
    std::env::set_current_dir("./addons").unwrap();
    assert_eq!(Project::find_root().unwrap(), root);

    hemtt_app::execute(
        &[
            "hemtt-app".to_string(),
            "project".to_string(),
            "version".to_string(),
        ],
        true,
    )
    .unwrap();
    hemtt_app::execute(&["hemtt-app".to_string(), "build".to_string()], false).unwrap();

    hemtt_pbo::test::pbo(
        File::open("addons/test_main.pbo").unwrap(),
        5,
        true,
        3,
        "0.1.0",
        "z\\test\\addons\\main",
        vec![
            234, 244, 222, 60, 227, 171, 217, 12, 216, 12, 168, 25, 137, 227, 200, 12, 237, 154,
            105, 243,
        ],
    );
}
