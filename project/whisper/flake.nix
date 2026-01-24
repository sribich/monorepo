{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  nixConfig = {
    substituters = [
      "https://cache.nixos.org/"
      "https://cache.nixos-cuda.org"
    ];
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
    }:
    let
      mkPkgs =
        repository: system: overlays:
        import repository {
          inherit system;
          overlays = overlays;
          config = {
            cudaSupport = true;
            allowUnfree = true;
          };
        };

      mkShell =
        system:
        let
          pkgs = mkPkgs nixpkgs system ([ ]);
        in
        import ./shell.nix {
          inherit self system pkgs;
        };
    in
    {
      devShells."x86_64-linux" = mkShell "x86_64-linux";
    };
}
