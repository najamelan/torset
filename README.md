Introduction
============

Tor is a great tool. You might want to create routers or servers which filter all passing traffic and send it through tor. On linux systems this can be done by creating the appropriate iptables rules. However one problem might arise: if users behind the firewall are already using tor/tails, their connections shouldn't be torified twice.

Torset lets you create the firewall sets with tor nodes (either by ipset or nftables variable) in order to create exception rules letting connections to tor nodes pass unhindered while you can redirect anything else. Torset reads a microdescriptor consensus file (the default one at /var/lib/tor/cached-microdesc-consensus or a user supplied one) and creates the ipset with the name you specify on the command line.

You can then use --match-set in iptables. See: 'man iptables-extensions | grep -x "   set" -A53' for more explanation.

Torset generates a restore file for ipset which you can then pipe to ipset. You can run torset periodically to update the set.

For nftables, a variable with the set is generated. You can store that in a file to be included in your nftables configuration.


Requirements
============

You will need:
- make
- rust compiler
- an up-to-date cached-microdesc-consensus file, usually available if you run tor.


Installation
============

Setup the required tools:
-------------------------

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Install cargo make: `cargo install cargo-make`

1. git clone https://github.com/najamelan/torset.git OR download the zip file from github
2. git checkout master                               OR extract  the zip file
3. cd torset
4. sudo cargo make install


Uninstallation
==============

Run "sudo make uninstall".


Usage
=====
running torset will tell you:

```
torset     : Generate or update an ipset or an nftables set of tornodes from the cached microdescriptor file.
version    : 19.1.19 (by Naja Melan <najamelan on autistici.org>)

usage      : torset help [subcommand]
             torset ipset    [set_name] [OPTIONS] | ipset
             torset nftables [var_name] [OPTIONS] > /etc/tornodes.conf

description: torset allows you to create firewall rules based on tor node ip addresses. Eg. If you want to transparently
             reroute traffic through tor, you would create a firewall rule to make sure no other outgoing connections
             get through, and you would not want to reroute traffic that already connects to tor, to avoid double torifying.

             With the help of the set generated from torset you can do this in the most common firewalls on linux. Firehol does
             not support ipsets with ports, so if you don't want to bother modifying the iptables rules, you can still generate
             an ipset without the --ports option to get ip addresses only.

             torset does not generate ip6 sets.

    help        Prints this message or the help of the given subcommand(s)
    ipset       Generate an ipset of tor nodes
    nftables    Generate an nftables set of tor nodes

    -h, --help                 Prints help information
    -i, --input <filename>     Where to read the consensus file from. Note that the default value usually requires
                               torset to be run as root. Special value accepted: 'stdin'. If you don't want torset to
                               run as root, you can pipe to stdin. [default: /var/lib/tor/cached-microdesc-consensus]
    -o, --output <filename>    Where to send the results. Special values accepted: 'stdout' and 'stderr' [default:
                               stdout]
    -p, --ports                Whether to make a set with both ip addresses and ports
    -V, --version              Prints version information
```

Bugs and limtations
===================

Hopefully no bugs... It is currently not configurable which type of ipset is created. It will be hash:ip,port or hash:ip, depending on whether you want ports or not, but that should be the best choice for most users.


Licence
=======

Unlicense
