% tlslb.conf(5)
% Raphael Peters <rappet@rappet.de>
% Mai 2025

# Name

**tlslb.conf** — TCP/TLS loadbalancer configuration file

# Description

The current behaviour of the loadbalancer can currently only be defined by the configuration file.

# Global Configuration

# Frontend Configuration

Multiple frontends can be defined in the configuration file.

`listen-address`
: Defines on which port/address the deamon should listen for a specific frontend

## Example 

```toml
[frontends.https]
listen-address = "[::]:8443"
type = "tls"
```

# Backend Configuration

`addresses`
: List of addresses to connect to

## Example

```toml
[backends."example.com"]
addresses = ["[3fff::1]:443", "[3fff::2]:443"]
```

# Files

*/etc/tlslb/tlslb.conf*

: main configuration file

*/run/tlslb/tlslb.sock*

: Configuration/monitoring interface


# Bugs

See GitHub Issues: <https://github.com/rappet/tlslb/issues>

# See Also

**tlslb(1)**