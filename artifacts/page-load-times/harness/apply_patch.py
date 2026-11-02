"""Add the timing tour to a scratch copy of the source. Usage: apply_patch.py <source copy>"""
import pathlib
import re
import shutil
import sys

src = pathlib.Path(sys.argv[1]) / "src"
here = pathlib.Path(__file__).parent


def swap(text, old, new):
    assert text.count(old) == 1, old
    return text.replace(old, new)


main = (src / "main.rs").read_text()
main = swap(main, "mod party;\n", "mod party;\nmod perf;\n")
main = swap(
    main,
    "fn main() -> eframe::Result {\n    let options = eframe::NativeOptions {\n",
    "fn main() -> eframe::Result {\n    perf::mark_main();\n"
    "    let options = eframe::NativeOptions {\n        vsync: !perf::enabled(),\n",
)
main = swap(
    main,
    "            design::theme::install(&cc.egui_ctx);\n"
    "            Ok(Box::<Splat>::default())\n",
    "            design::theme::install(&cc.egui_ctx);\n"
    "            perf::mark_creator(cc);\n"
    "            let app = Box::<Splat>::default();\n"
    "            perf::mark_app();\n"
    "            Ok(app)\n",
)
main = swap(
    main,
    "    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {\n",
    "    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {\n"
    "        perf::begin(self, ctx, frame);\n",
)
main = swap(
    main,
    "        self.remember();\n    }\n}\n",
    "        self.remember();\n        perf::end(self, ctx);\n    }\n}\n",
)
(src / "main.rs").write_text(main)

# The tour needs the checks that the visual tests use.
for name in ["library", "settings", "party", "encounters", "lookup", "campaign_content"]:
    path = src / f"{name}.rs"
    text = path.read_text()
    text, count = re.subn(r"    #\[cfg\(test\)\]\n(    pub fn busy\(&self\))", r"\1", text)
    assert count == 1, name
    path.write_text(text)

# One world with the largest saved run, for the Run page. The real campaign has no runs.
tests = src / "visual_tests" / "mod.rs"
tests.write_text(
    tests.read_text()
    + """
#[test]
#[ignore]
fn perf_world() {
    let root = PathBuf::from(std::env::var("SPLAT_PERF_WORLD").expect("SPLAT_PERF_WORLD"));
    crate::hermetic::closed::repeatable_uuids(1 << 64);
    let cache = root.join("character-cache");
    crate::hermetic::closed::use_cache_root(cache.clone());
    World::build(&root.join("world"), &cache, "perf", Seed::LargeRosters).expect("world");
}
"""
)
shutil.copy(here / "perf.rs", src / "perf.rs")
