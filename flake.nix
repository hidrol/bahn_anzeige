{
  description = "Bahnanzeige website";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
    in
    {
      packages = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./default.nix { };
      });
      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix { };
      });
      nixosModules.default = { config, lib, pkgs, ... }:
        with lib;
        let
          cfg = config.bahn_anzeige;
        in
        {
          options.bahn_anzeige = {
            enable = mkEnableOption "Enable the Douglas Adams quotes service";

            # logLevel = mkOption {
            #   type = with types; enum [ "DEBUG" "INFO" "ERROR" ];
            #   example = "DEBUG";
            #   default = "INFO";
            #   description = "log level for this application";
            # };

            # port = mkOption {
            #   type = types.port;
            #   default = 8080;
            #   description = "port to listen on";
            # };

            package = mkOption {
              type = types.package;
              default = self.packages.${pkgs.system}.default;
              description = "package to use for this service (defaults to the one in the flake)";
            };
          };
          config = mkIf cfg.enable {
            systemd.services.bahn_anzeige = {
              description = "Bahnanzeige website";
              wantedBy = [ "multi-user.target" ];

              serviceConfig = {
                DynamicUser = "yes";
                #ExecStart = "${cfg.package}/bin/bahn_anzeige --slog-level=${cfg.logLevel} --addr=:${toString cfg.port}";
                ExecStart = "${cfg.package}/bin/bahn_anzeige";
                Restart = "on-failure";
                RestartSec = "5s";
              };
            };
          };
        };
    };
}
