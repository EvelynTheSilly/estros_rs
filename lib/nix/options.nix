{ lib, ... }:
{
  options.estros.inits = lib.mkOption {
    type = lib.types.attrsOf (lib.types.attrsOf (lib.types.submodule {
      options = {
        name = lib.mkOption { type = lib.types.str; };
        pkg = lib.mkOption { type = lib.types.package; };
      };
    }));
    default = { };
    description = "Named init definitions grouped by source";
  };

  config.systems = [
    "x86_64-linux"
    "aarch64-linux"
    "aarch64-darwin"
  ];
}
