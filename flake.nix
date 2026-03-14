{
  description = "MCP server for WezTerm terminal interaction";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    crane,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      fenixPkgs = fenix.packages.${system};
      toolchain = fenixPkgs.stable.toolchain;
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

      src = craneLib.cleanCargoSource ./.;

      commonArgs = {
        inherit src;
        strictDeps = true;
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      mcp-server-wezterm = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
        });
    in {
      packages.default = mcp-server-wezterm;

      devShells.default = craneLib.devShell {
        packages = [
          pkgs.wezterm
          pkgs.cargo-nextest
          pkgs.cargo-deny
          pkgs.just
          pkgs.taplo
          pkgs.typos
        ];
      };
    });
}
