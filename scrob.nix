{ lib
, rustPlatform
}:
rustPlatform.buildRustPackage {
  name = "scrob";

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    # Allow dependencies to be fetched from git and avoid having to set the outputHashes manually
    allowBuiltinFetchGit = true;
  };

  meta = with lib; {
    description = "Pluggable, open-source audio scrobbler, for crustaceans.";
    homepage = "https://github.com/shymega/scrob";
    license = licenses.mit;
  };
}
