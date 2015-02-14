Introduction
============

Tor is a great tool. You might want to create routers or servers which filter all passing traffic and send it through tor. On linux systems this can be done by creating the appropriate iptables rules. However one problem might arise: if users behind the firewall are already using tor/tails, their connections shouldn't be torified twice.

Torset is a tool that lets you create an ipset (see man ipset) for iptables to create the exception rules needed to let connections to tor nodes pass unhindered, while you can redirect anything else. Torset reads a microdescriptor consensus file (the default one at /var/lib/tor/cached-microdesc-consensus or a user supplied one) and creates the ipset with the name you specify on the command line.

You can then use --match-set in iptables. See: 'man iptables-extensions | grep -x "   set" -A53' for more explanation.

In practice torset generates a restore file for ipset which you can then pipe to ipset. You can run torset periodically to update the set.


Requirements
============

You will need:
- make
- a c++ compiler
- ipset
- automake in order to facilitate creating the configure scripts and the makefiles.

On a debian like system you can satisfy the requirements by running:

```sudo apt install build-essential ipset automake```


Installation
============


1. git clone https://github.com/najamelan/torset.git OR download the zip file from github
2. git checkout master                               OR extract  the zip file
3. cd torset
4. autoreconf --install -Wall
5. ./configure
6. sudo make install

If you want a tarball, you can run "make distcheck"
If you want to uninstall, run "sudo make uninstall"


Usage
=====
running torset will tell you:

torset  : create or update an ipset from a tor microdescriptor consensus file.
Version : 2015.02.14.BETA
Usage   : torset setname [ consensusFile ] | sudo ipset restore

consensusFile defaults to '/var/lib/tor/cached-microdesc-consensus' in which case torset will require root privileges.



Bugs and limtations
===================

Torset is currently an beta release. There is no unit testing and no error handling. Use at your own risk. This software will work on systems with linux kernels only.



Licence
=======

public domain
