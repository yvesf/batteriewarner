{
  outputs = { ... }:
    let
      batteriewarner = { lib, rustPlatform, makeWrapper }:
        rustPlatform.buildRustPackage rec {
          name = "batteriewarner-${version}";
          version = "2022-08-25";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          buildInputs = [ makeWrapper ];
          meta = {
            description = "Display low battery status using the power led of Thinkpads X-series";
            homepage = https://github.com/yvesf/batteriewarner;
            platforms = lib.platforms.all;
          };
        };
    in
    {
      nixosModule = { config, lib, pkgs, ... }:
        let
          package = pkgs.callPackage batteriewarner { };
          cfg = config.services.batteriewarner;
        in
        {
          options.services.batteriewarner.enable = lib.mkEnableOption "Batteriewarner";
          config = lib.mkIf cfg.enable {
            systemd.services.batteriewarner = {
              enable = true;
              description = "Batteriewarner";
              wantedBy = [ "multi-user.target" ];
              serviceConfig.ExecStart = "${package}/bin/batteriewarner";
            };
          };
        };
    };
}

