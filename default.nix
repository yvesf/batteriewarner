{ pkgs ? (import <nixpkgs> {}) }:
let
  rustPlatform = pkgs.rustPlatform;
  makeWrapper = pkgs.makeWrapper;
in 
  rustPlatform.buildRustPackage rec {
    name = "batteriewarner-${version}";
    version = "2017-09-15";
    src = ./.;

    cargoSha256 = "sha256:1rf1sckjpjwdvllx3aapw646wl522j9cn7zx4bah805d6ak9plls";

    buildInputs = [ makeWrapper ];

    meta = {
      description = "Display low battery status using the power led of Thinkpads X-series";
      homepage = https://github.com/yvesf/batteriewarner;
      platforms = pkgs.lib.platforms.all;
    };
  }

