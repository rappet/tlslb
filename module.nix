{
  config,
  lib,
  pkgs,
  ...
}:
with lib;

let
  cfg = config.services.tlslb;
in
{
  options.services.tlslb = {
    enable = mkEnableOption "tlslb config";

    package = mkPackageOption pkgs "tlslb" { };

    user = lib.mkOption {
      type = lib.types.str;
      default = "tlslb";
      description = "User account under which tlslb runs.";
    };

    group = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = "tlslb";
      description = "Group account under which tlslb runs.";
    };

    config = mkOption {
      type = types.lines;
      description = ''
        Configuration file, see tlslb.conf(5)
      '';
    };
  };

  config = mkIf cfg.enable {
    systemd.services.tlslb = {
      description = "TLS loadbalancer daemon";

      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      serviceConfig = {
        User = cfg.user;
        Group = cfg.group;

        ExecStart = "${cfg.package}/bin/tlslb --config-file=${pkgs.writeText "tlslb.conf" cfg.config}";
        Restart = "always";
        RuntimeDirectory = "tlslb";
        # upstream hardening options
        NoNewPrivileges = true;
        ProtectHome = true;
        ProtectSystem = "strict";
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectControlGroups = true;
        SystemCallFilter = "~@cpu-emulation @keyring @module @obsolete @raw-io @reboot @swap @sync";
        # needed in case we bind to port < 1024
        AmbientCapabilities = "CAP_NET_BIND_SERVICE";
      };
    };

    users.users = lib.optionalAttrs (cfg.user == "tlslb") {
      tlslb = {
        group = cfg.group;
        isSystemUser = true;
      };
    };

    users.groups = lib.optionalAttrs (cfg.group == "tlslb") {
      tlslb = { };
    };
  };
}
