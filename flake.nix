{
  description = "estros devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        cross = pkgs.pkgsCross.aarch64-embedded;
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnsupportedSystem = true;
        };
        toolchainToml = fromTOML (builtins.readFile ./rust-toolchain.toml);

        toolchain = toolchainToml.toolchain;

        rust = pkgs.rust-bin.fromRustupToolchain {
          channel = toolchain.channel;
          components = toolchain.components or [ ];
          targets = toolchain.targets or [ ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rust
            bacon
            pkg-config
            openssl
            cross.buildPackages.gcc
            cross.buildPackages.binutils
            (if system != "darwin" then cross.buildPackages.gdb else { })
            pkgs.qemu
            pkgs.cmake
            pkgs.mask
            pkgs.cloc
          ];
          shellHook = ''
            echo "AArch64 bare-metal dev shell ready!"
            echo "Toolchain prefix: aarch64-none-elf-"
            nu -e "alias cloc = cloc --vcs git"
          '';
        };
      }
    );
}
