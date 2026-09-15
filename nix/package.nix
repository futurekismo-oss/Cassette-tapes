{
  rustPlatform,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage {
  pname = "tapes";
  version = "0.1.5";

  src = fetchFromGitHub {
    owner = "futurekismo-oss";
    repo = "Cassette-tapes";
    rev = "490d80ec6788e62672b3bb197b461a6bc5605eb5";
    hash = "sha256-ba3ebCvslLbv8peYjG3vC/5cKRM8+QztmhkY9IWXRZQ=";
  };

  cargoHash = "";
}
