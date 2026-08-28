{
  description = "fastannoy - typos that are actually annoying";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        # Common command typos this package intentionally shadows.
        # 'sl' is deliberately excluded: it's already an established,
        # well-known joke package (steam locomotive) elsewhere in nixpkgs.
        typos = [ "gti" "gerp" "sudp" "cst" "vom" ];
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "fastannoy";
          version = cargoToml.package.version;

          src = pkgs.lib.cleanSource ./.;

          cargoLock.lockFile = ./Cargo.lock;

          postInstall = ''
            for typo in ${pkgs.lib.concatStringsSep " " typos}; do
              ln -s "$out/bin/fastannoy" "$out/bin/$typo"
            done
          '';

          meta = with pkgs.lib; {
            description = "Typos that are actually annoying";
            homepage = "https://github.com/CallMeAlphabet/fastannoy";
            license = licenses.asl20;
            platforms = platforms.linux;
            mainProgram = "fastannoy";
          };
        };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [ pkgs.cargo pkgs.rustc ];
        };
      });
}
