{ lib, rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "resty-kv";
  version = "0.1.0";

  src = builtins.path { path = ./.; };

  cargoHash = "sha256-nC0Ks+kCYo1dvRzWrA0tlnYY4/nUo3loE8ObwEnhbNc=";

  meta = with lib; {
    description = "A simple key-value store based on Sqlite with an HTTP API";
    homepage = "https://github.com/auxves/resty-kv";
    license = licenses.gpl3;
  };
}
