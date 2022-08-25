{
  inputs.nixpkgs.url = "nixpkgs/nixos-20.09";
  outputs = { self, nixpkgs }:
    with import nixpkgs { system = "x86_64-linux"; };
    {
      defaultPackage.x86_64-linux = callPackage ./. { };
      nixosModule = { config }: { imports = [ ./module.nix ]; };
    };
}

