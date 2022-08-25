{ config, lib, pkgs, ... }:
let
  package = pkgs.callPackage ./. { };
  cfg = config.programs.batteriewarner;
in
{
  options.programs.batteriewarner = {
    enable = lib.mkEnableOption "Batteriewarner";
  };
  config = lib.mkIf cfg.enable {
    systemd.services.batteriewarner = {
      enable = true;
      description = "Batteriewarner";
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        ExecStart = "${package}/bin/batteriewarner";
      };
    };
  };
}

