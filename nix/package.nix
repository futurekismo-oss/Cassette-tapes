{
  rustPlatform,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage {
  pname = "tapes";
  version = "0.1.5";

  src = ../.;

  cargoHash = "sha256-yIJoXuRdUqN4E+V/SxFM0d9d4FLqWg9W9V4d+B2PakA=";
}
