pub mod enums;
pub mod structs;

pub use enums::*;
pub use structs::*;

use std::collections::HashMap;

fn main() {
    let mut run_deps: HashMap<String, u64> = HashMap::new();
    run_deps.insert(String::from("nodejs"), 20220802);
    run_deps.insert(String::from("node-gyp"), 20220802);

    let mut build_deps: HashMap<String, u64> = HashMap::new();
    build_deps.insert(String::from("glibc"), 20220519);

    let package: Package = Package {
        name: String::from("code"),
        version: String::from("20220802"),
        architecture: Architecture::X86_64,
        built_time: 20220519,
    };

    let deps = Dependency {
        run: Some(run_deps),
        build: Some(build_deps),
        test: None,
    };

    let build = r#"
sudo pacman -Syu --noconfirm
sudo pacman -S neofetch --noconfirm
mkdir -p test/{test1,test2/test4}
"#
    .to_string();

    let manifest: Manifest = Manifest {
        package: package,
        dependencies: Some(deps),
        build: BuildScript { script: build },
    };

    manifest.build.build();
}
