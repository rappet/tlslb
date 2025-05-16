% tlslb.conf(5)
% Raphael Peters <rappet@rappet.de>
% Mai 2025

# NAME

**tlslb.conf** — TCP/TLS loadbalancer configuration file

# DESCRIPTION

The current behaviour of the loadbalancer can currently only be defined by the configuration file.

# GLOBAL CONFIGURATION

Currently, this is empty.

# FRONTEND CONFIGURATION

Multiple frontends can be defined in the configuration file.
But in the current version, this is only limited to one frontend called "https".

`listen-address`
: Defines on which port/address the deamon should listen for a specific frontend

## Example 

```toml
[frontends.https]
listen-address = "[::]:8443"
type = "tls"
```

# BACKEND CONFIGURATION

`addresses`
: List of addresses to connect to

> Either a socket address like _192.0.2.0:8443_ or _[2001:db8::1]:8443_
> or an address like _backend.tld:8443_ that will result in a DNS lookup.
> The set of all addresses/DNS responses will be used.
> If one address appears multiple times during the lookup, it will only be used once.

`preconnect_count`
: Count of connections that will be held idle in the pool as preparation for new connections

> The default value is 0, which means, that for each connecting client a new connection
> to the backend will be made.
> If the value is greater than 0, connections will be made in advance and used for future
> connections on the frontend, which can result in faster round trip times.

## Example

```toml
[backends."example.com"]
addresses = ["[2001:db8::1]:443", "[2001:db8::2]:443", "example.local:443"]
```

# FILES

*/etc/tlslb/tlslb.conf*

: main configuration file

*/run/tlslb/tlslb.sock*

: Configuration/monitoring interface


# BUGS

See GitHub Issues: <https://github.com/rappet/tlslb/issues>

# SEE ALSO

**tlslb(1)**