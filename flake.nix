{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = inputs: let
    systems = ["aarch64-linux" "aarch64-darwin" "x86_64-darwin" "x86_64-linux"];
    forSystems = inputs.nixpkgs.lib.genAttrs systems;
    pkgsFor = system: (import inputs.nixpkgs {inherit system;});
    genPkgs = func: (forSystems (system: func (pkgsFor system)));
  in {
    packages = genPkgs (pkgs: {
      my-package = pkgs.rustPlatform.buildRustPackage {
        pname = "my-package";
        version = "0.1.0";
        # nativeBuildInputs = with pkgs; [];
        # buildInputs = with pkgs; [ ];
        src = ./.;
        hash = pkgs.lib.fakeHash;
        cargoHash = pkgs.lib.fakeHash;
      };
      default = inputs.self.packages.${pkgs.system}.my-package;
    });
    devShells = genPkgs (pkgs: {
      default = pkgs.mkShell {
        inputsFrom = [inputs.self.packages.${pkgs.system}.default];
        packages = with pkgs; [
          rustPackages.clippy
          rust-analyzer
          rustfmt
        ];
      };
    });
  };
}
