{
  description = "Simple TCP/TLS load balancer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      systems,
    }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      overlays.default = import ./overlay.nix;

      formatter = eachSystem (system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style);

      packages = eachSystem (system: {
        tlslb = nixpkgs.legacyPackages.${system}.callPackage ./tlslb.nix { };
        default = self.packages.${system}.tlslb;
      });

      nixosModules.tlslb = import ./module.nix;
    };
}
