{ inputs, ... }:
{
  perSystem =
    { system, ... }:
    let
      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [ inputs.rust-overlay.overlays.default ];
      };

      rustNightly = pkgs.rust-bin.selectLatestNightlyWith (
        t:
        t.default.override {
          extensions = [ "llvm-tools" ];
        }
      );

      rustStable = pkgs.rust-bin.stable.latest.default;
    in
    {
      _module.args = {
        inherit
          pkgs
          rustNightly
          rustStable
          ;
      };
    };
}
