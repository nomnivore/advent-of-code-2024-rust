# .envrc: use flake
{
  description = "DevShell flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [

          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
            openssl
          ];
          LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [ openssl ];
        };
      }
    );
}
