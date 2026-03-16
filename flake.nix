{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = 
  {self, nixpkgs}:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {inherit system; };
      nix-reduce = pkgs.rustPlatform.buildRustPackage {
        pname = "nix-reduce";
        version = "0.1";
        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };
    in
    {
      packages."${system}".default = nix-reduce;
    };
}
