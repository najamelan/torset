Introduction [![Build Status](https://travis-ci.org/najamelan/torset.svg?branch=master)](https://travis-ci.org/najamelan/torset)[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
============

Tor is a great tool. You might want to create routers or servers which filter all passing traffic and send it through tor. On linux systems this can be done by creating the appropriate iptables rules. However one problem might arise: if users behind the firewall are already using tor/tails, their connections shouldn't be torified twice.

Torset lets you create the firewall sets with tor nodes (either by ipset or nftables variable) in order to create exception rules letting connections to tor nodes pass unhindered while you can redirect anything else. Torset reads a microdescriptor consensus file (the default one at /var/lib/tor/cached-microdesc-consensus or a user supplied one) and creates the ipset with the name you specify on the command line.

You can then use --match-set in iptables. See: 'man iptables-extensions | grep -x "   set" -A53' for more explanation.

Torset generates a restore file for ipset which you can then pipe to ipset. You can run torset periodically to update the set.

For nftables, a variable with the set is generated. You can store that in a file to be included in your nftables configuration.

After compilation this program doesn't require you to install any scripting languages like python, so it can be a good option for installation on routers. It performant as well.

**If you can't compile rust, check out the C++ branch of this repository. It only supports ipset, not nftables.**

libtorset
=========

The crate is split between a library and a cli frontend, so if you need microdescriptor parsing, you can link against the library to read all fields of the microdescritor. Just add it as a dependency in your toml file.

Then just import: `use libtorset::microdescriptor::MicroDescriptor;`


Requirements
============

You will need:
- rust compiler
- cargo-make
- an up-to-date cached-microdesc-consensus file, usually available if you run tor.


Installation
============

Setup the required tools:
-------------------------

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Install cargo make: `cargo install cargo-make`

1. `git clone https://github.com/najamelan/torset.git` OR download the zip file from github
2. `git checkout master`                               OR extract  the zip file
3. `cd torset`
4. `cargo make install`

`cargo make install` will prompt for your password as it uses sudo. It will install to /usr/local/bin. It will also require internet access to download rust dependencies.

Now you can deploy the binary version as long as you don't change architecture. You can find it in the folder `target/release/`Don't forget to deploy the man page. You can cross compile to a whole series of targets with rust.


Uninstallation
==============

Run `cargo make uninstall`.


Usage
=====
running torset will tell you:

```
torset     : Generate or update an ipset or an nftables set of tornodes from the cached microdescriptor file.
version    : 19.1.19 (by Naja Melan <najamelan on autistici.org>)

usage      : torset help     [subcommand]
             torset ipset    [set_name] [OPTIONS] | ipset restore
             torset nftables [var_name] [OPTIONS] > /etc/tornodes.conf

description: torset creates an ipset or an nftables variable containing all tor nodes, for use in firewalls.you
             If you want to transparently reroute traffic through tor, you would create a firewall rule to make
             sure no other outgoing connections get through, and you would not want to reroute traffic that already
             connects to tor, to avoid double torifying.

             With the help of the set generated from torset you can do this in the most common firewalls on linux. Firehol does
             not support ipsets with ports, so if you don't want to bother modifying the iptables rules, you can still generate
             an ipset without the --ports option to get ip addresses only.

             torset does not generate ip6 sets.

    help        Prints this message or the help of the given subcommand(s)
    ipset       Generate an ipset of tor nodes
    nftables    Generate an nftables set of tor nodes

    -h, --help                 Prints help information
    -i, --input <filename>     Where to read the consensus file from. Note that the default value usually requires
                               torset to be run as root. If you don't want torset to run as root, you can pipe to stdin.
                               Special value accepted: 'stdin'.  [default: /var/lib/tor/cached-microdesc-consensus]
    -o, --output <filename>    Where to send the results. Special values accepted: 'stdout' and 'stderr' [default:
                               stdout]
    -p, --ports                Whether to make a set with both ip addresses and ports
    -V, --version              Prints version information
```

Systemd
=======
You can ofcourse run torset as a service. Here are some example unit files:


torset.service:
```ini
[Unit]

    Description         = Creates the ipset of tor nodes
    DefaultDependencies = false


[Service]

    Type            = oneshot
    RemainAfterExit = true
    ExecStart       = /usr/bin/sh -c "set -o pipefail && /usr/local/bin/torset ipset tornodes --ports | ipset restore"
    ExecReload      = /usr/bin/sh -c "set -o pipefail && /usr/local/bin/torset ipset tornodes --ports | ipset restore"
    ExecStop        = /usr/bin/ipset destroy tornodes
    StandardError   = journal+console

[Install]

    RequiredBy      = firehol.service
```

torset-watch.path:
```ini
[Unit]

    Description = Watching /var/lib/cached-microdesc-consensus for changes (to restart torset.service).


[Path]

    PathChanged = /var/lib/tor/cached-microdesc-consensus
    Unit        = torset-watch.service

[Install]

    WantedBy    = torset.service
```

torset-watch.service:
```ini
[Unit]

    Description = torset-watch.service: Relaunch torset if the cached microdescriptor file changes.


[Service]

    Type        = oneshot
    ExecStart   = /usr/bin/systemctl reload torset.service

[Install]

    WantedBy    = torset.service
```

Bugs and limtations
===================

Hopefully no bugs... It is currently not configurable which type of ipset is created. It will be hash:ip,port or hash:ip, depending on whether you want ports or not, but that should be the best choice for most users.

Please report any bugs to http://github.com/najamelan/torset/issues


Licence
=======

Unlicense
