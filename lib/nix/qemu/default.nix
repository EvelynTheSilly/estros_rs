{ inputs, ... }:
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  perSystem =
    {
      inputs',
      self',
      pkgs,
      system,
      ...
    }:
    let
      limine = pkgs.limine-full;
      ovmf = pkgs.pkgsCross.aarch64-multiplatform.OVMF.fd;
      cross = pkgs.pkgsCross.aarch64-embedded;
      isLinux = system != "aarch64-darwin";

      build = import ./_build.nix {
        pkgs = pkgs;
        kernel = self'.packages.kernel_elf;
        inherit limine ovmf;
      };
      mkScript = import ./_scripts.nix;

      run =
        mkScript {
          inherit pkgs;
          name = "estros-run";
          inherit (build) efiVars diskImage;
        };
      debug =
        mkScript {
          inherit pkgs;
          name = "estros-debug";
          inherit (build) efiVars diskImage;
          extraFlags = "-S -s";
        };
    in
    {
      packages = {
        inherit run debug;
        default = run;

        krun = pkgs.writeShellScriptBin "krun" ''
          exec nix run .#run -- "$@"
        '';
        kdebug = pkgs.writeShellScriptBin "kdebug" ''
          nix build .#gdb
          ${pkgs.alacritty}/bin/alacritty -e ./result/bin/gdb &
          gdb_pid=$!
          trap 'kill $gdb_pid 2>/dev/null' EXIT
          exec nix run .#debug -- "$@"
        '';
      } // pkgs.lib.optionalAttrs isLinux {
        gdb = pkgs.writeShellScriptBin "gdb" ''
          kernel_path=$(nix build .#kernel_elf --no-link --print-out-paths)
          init_path=$(nix build .#init --no-link --print-out-paths)
          tmpgdbinit=$(mktemp)
          trap 'rm -f "$tmpgdbinit"' EXIT
          sed \
            -e "s|KERNEL_ELF_PATH|$kernel_path/kernel.elf|g" \
            -e "s|INIT_ELF_PATH|$init_path/init.elf|g" \
            ${./gdbinit} > "$tmpgdbinit"
          exec ${cross.buildPackages.gdb}/bin/aarch64-none-elf-gdb -ix "$tmpgdbinit" "$@"
        '';
      };
    };
}
