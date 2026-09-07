{ inputs, ... }:
let
  ccPackage = import ../../lib/c;

  mkInit =
    pkgs:
    buildType:
    let
      cc = ccPackage { inherit pkgs; };
      optFlag = if buildType == "debug" then "-O0" else "-O2";
    in
    pkgs.stdenv.mkDerivation {
      name = "thread_test";
      src = ./.;
      nativeBuildInputs = [
        cc.packages.aarch64-estros-binutils
      ];
      buildPhase = ''
        aarch64-estros-gcc ${optFlag} main.c -o init.elf
      '';
      installPhase = ''
        mkdir $out
        cp init.elf $out
      '';
    };

  pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
in
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  flake =
    { ... }:
    {
      estros.inits.thread_test = {
        release = {
          name = "thread_test";
          pkg = mkInit pkgs "release";
        };
        debug = {
          name = "thread_test";
          pkg = mkInit pkgs "debug";
        };
      };
    };
}
