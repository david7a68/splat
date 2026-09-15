{pkgs, ...}: {
  languages.rust = {
    enable = true;
    channel = "stable";
    version = "1.95.0";
    components = ["rust-src" "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer"];

    # Linux x86_64 only: wild rides on -C linker-features=-lld, which devenv
    # has to gate behind -Z unstable-options on aarch64, and that would force
    # channel = "nightly" above. So aarch64 (the CI/deploy ARM runners) gets
    # mold instead of the default bfd linker, which was costing roughly 170s
    # per CI run linking the test binaries. devenv asserts at most one of
    # mold/lld/wild, so these two predicates must stay mutually exclusive, and
    # it separately asserts wild is Linux-only.
    wild.enable = pkgs.stdenv.hostPlatform.isLinux && pkgs.stdenv.hostPlatform.isx86_64;
    mold.enable = pkgs.stdenv.hostPlatform.isLinux && !pkgs.stdenv.hostPlatform.isx86_64;
  };
}
