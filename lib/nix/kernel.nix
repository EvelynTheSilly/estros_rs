{ inputs, self, ... }:
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  perSystem =
    {
      self',
      pkgs,
      ...
    }:
    let
      inits = self.estros.inits;
      releaseInit = self.lib.buildInit { init = inits.c_hello_world.release; };
      debugInit = self.lib.buildInit { init = inits.c_hello_world.debug; };
    in
    {
      packages = {
        kernel_elf = self.lib.buildKernel {
          init = releaseInit;
          inherit pkgs;
          rust = self'.packages.rust;
        };
        kernel_elf_debug = self.lib.buildKernel {
          init = debugInit;
          inherit pkgs;
          buildType = "debug";
          rust = self'.packages.rust;
        };
      };
    };
}
