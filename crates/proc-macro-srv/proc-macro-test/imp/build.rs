//! This teaches cargo about our cfg(rust_analyzer)

fn main() {
    // Constant output, no inputs. Declare an escape-hatch env so cargo doesn't
    // fall back to the whole-package mtime scan (which re-runs this script on
    // every fresh checkout); set it to any value to force a re-run.
    println!("cargo:rerun-if-env-changed=PROC_MACRO_TEST_FORCE_RERUN");
    println!("cargo:rustc-check-cfg=cfg(rust_analyzer)");
}
