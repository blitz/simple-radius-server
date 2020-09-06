{ nixpkgs, simple-radius-server }:

import "${nixpkgs}/nixos/tests//make-test-python.nix" ({ pkgs, ... }:
  let radiusSecret = "notverysecret";
  in {
    name = "simple-radius-server-test";

    nodes = {
      server = { pkgs, ... }: {

        networking.firewall.allowedUDPPorts = [ 1812 ];

        systemd.services.radius = {
          description = "The Simple RADIUS Server";
          path = [ simple-radius-server ];

          after = [ "network.target" ];
          wantedBy = [ "multi-user.target" ];

          serviceConfig = {
            Restart = "on-failure";

            # Paranoia Mode
            DynamicUser = true;
            AmbientCapabilities = [];
            CapabilityBoundingSet = [];
            DevicePolicy = "closed";
            LockPersonality = true;
            MemoryDenyWriteExecute = true;
            NoNewPrivileges = true;
            PrivateDevices = true;
            PrivateMounts = true;
            PrivateTmp = true;
            PrivateUsers = true;
            ProtectClock = true;
            ProtectControlGroups = true;
            ProtectHome = true;
            ProtectHostname = true;
            ProtectKernelLogs = true;
            ProtectKernelModules = true;
            ProtectKernelTunables = true;
            ProtectSystem = "strict";
            RemoveIPC = true;
            RestrictAddressFamilies = [ "AF_INET" "AF_INET6" ];
            RestrictNamespaces = true;
            RestrictRealtime = true;
            RestrictSUIDSGID = true;
            SystemCallArchitectures = "native";
          };

          script = let
            authScript = pkgs.writeShellScript "radius-auth" ''
              set -eu

              USER=$1
              PASS=$2

              if [ "$USER" == "klaus" -a "$PASS" == "correctpassword" ]; then
                exit 0
              else
                exit 1
              fi
            '';
          in ''
            exec simple-radius-server -vvv "${radiusSecret}" "${authScript}"
          '';
        };
      };

      client = { pkgs, ... }: {
        environment.systemPackages = [
          pkgs.freeradius # for radclient
        ];
      };
    };

    testScript = { nodes, ... }: ''
      server.wait_for_unit("radius.service")

      client.wait_for_unit("network-online.target")
      client.fail(
          """
            echo "User-Name=klaus,User-Password=wrongpassword" | radclient -P udp server:1812 auth ${radiusSecret}
          """
      )
      client.succeed(
          """
            echo "User-Name=klaus,User-Password=correctpassword" | radclient -P udp server:1812 auth ${radiusSecret}
          """
      )
    '';
  })
