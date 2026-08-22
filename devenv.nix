{
  pkgs,
  lib,
  config,
  ...
}:
{
  # https://devenv.sh/languages/
  languages = {
    rust = {
      enable = true;
      components = [
        "rustc"
        "cargo"
        "clippy"
        "rustfmt"
        "rust-analyzer"
      ];
    };
  };

  # See full reference at https://devenv.sh/reference/options/
}
