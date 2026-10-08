//! Runs every program in `examples/`. One with a sibling `.out` file must print exactly that;
//! one without must only run without error. `// test: skip` marks programs that sleep or read
//! input. `UPDATE_SNAPSHOTS=1` rewrites the `.out` files.

use meowscript::{BufferHost, Error, Host, Interpreter, Result};
use std::path::{Path, PathBuf};

#[derive(Clone)]
struct ExampleHost(BufferHost);

impl Host for ExampleHost {
    fn print(&mut self, line: &str) {
        self.0.print(line);
    }

    fn read_file(&mut self, path: &str) -> Result<String> {
        std::fs::read_to_string(path)
            .map_err(|e| Error::io(format!("could not read `{path}`: {e}")))
    }

    fn file_exists(&mut self, path: &str) -> bool {
        Path::new(path).exists()
    }

    fn random_seed(&mut self) -> u64 {
        1
    }
}

fn examples_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("examples")
}

#[test]
fn examples_run_and_match_their_snapshots() {
    let dir = examples_dir();
    let mut programs: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("examples directory")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "meow"))
        .collect();
    programs.sort();
    assert!(
        !programs.is_empty(),
        "no examples found in {}",
        dir.display()
    );

    let update = std::env::var_os("UPDATE_SNAPSHOTS").is_some();
    let mut failures = Vec::new();
    let mut checked = 0;

    for path in programs {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let source = std::fs::read_to_string(&path).unwrap();
        if source.contains("// test: skip") {
            continue;
        }
        checked += 1;

        let host = ExampleHost(BufferHost::new());
        let mut interp = Interpreter::new(host.clone());
        interp.set_base_dir(Some(dir.to_string_lossy().into_owned()));

        if let Err(e) = interp.run(&source) {
            failures.push(format!(
                "{name} failed:\n{}",
                e.render(&source, Some(&name))
            ));
            continue;
        }

        let snapshot = path.with_extension("out");
        let actual = {
            let mut s = host.0.output();
            s.push('\n');
            s
        };
        if update {
            std::fs::write(&snapshot, &actual).unwrap();
        } else if snapshot.exists() {
            let expected = std::fs::read_to_string(&snapshot)
                .unwrap()
                .replace("\r\n", "\n");
            if expected != actual {
                failures.push(format!(
                    "{name} printed something different from {}:\n--- expected\n{expected}--- actual\n{actual}",
                    snapshot.file_name().unwrap().to_string_lossy()
                ));
            }
        }
    }

    assert!(checked > 0, "every example was skipped");
    assert!(failures.is_empty(), "\n{}", failures.join("\n\n"));
}
