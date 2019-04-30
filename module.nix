{ config, lib, pkgs, ... }:
let
  rustPlatform = pkgs.rustPlatform;
  makeWrapper = pkgs.makeWrapper;
  batteriewarner = rustPlatform.buildRustPackage rec {
    name = "batteriewarner-${version}";
    version = "2017-09-15";
    src = ./.;

    doCheck = false;

    cargoSha256 = "sha256:0jacm96l1gw9nxwavqi1x4669cg6lzy9hr18zjpwlcyb3qkw9z7f";

    buildInputs = [ makeWrapper ];

    RUST_SRC_PATH = rustPlatform.rustcSrc;

    installPhase = ''
      mkdir -p $out/bin
      cp -p target/release/batteriewarner $out/bin/
      wrapProgram $out/bin/batteriewarner --set RUST_SRC_PATH "$RUST_SRC_PATH"
    '';

    meta = with lib; {
      description = "Display low battery status using the power led of Thinkpads X-series";
      homepage = https://github.com/yvesf/batteriewarner;
      platforms = platforms.all;
    };
  };
in
{
    config = {
        systemd.services.batteriewarner = {
            enable = true;
            description = "Batteriewarner";
            wantedBy = [ "multi-user.target" ];
            serviceConfig = {
                ExecStart = "${batteriewarner}/bin/batteriewarner";
            };
        };
    };
}

