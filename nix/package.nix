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
    rev = "bef75c5de02cb5dae78c48212ac24bce032d9bf3";
    hash = "sha256-vhGwhU/6HTzb+CGyAB85JiHy047EwlPRUN0L+FW2PJM=";
  };

  cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
}
