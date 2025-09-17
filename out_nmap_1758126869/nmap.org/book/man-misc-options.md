---
title: "Miscellaneous Options | Nmap Network Scanning"
source_url: https://nmap.org/book/man-misc-options.html
fetched_at: 2025-09-17T16:36:18.329444+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 15. Nmap Reference Guide](https://nmap.org/book/man.html)
* Miscellaneous Options

[Prev](https://nmap.org/book/man-output.html)

[Next](https://nmap.org/book/man-runtime-interaction.html)

Miscellaneous Options
----------

This section describes some important (and not-so-important) options that don't really fit anywhere else.

`-6` (Enable IPv6 scanning) []()[ ]()

[Nmap has IPv6 support for its most popular features. Ping scanning, port scanning, version detection, and the Nmap Scripting Engine all support IPv6. The command syntax is the same as usual except that you also add the `-6` option. Of course, you must use IPv6 syntax if you specify an address rather than a hostname. An address might look like `3ffe:7501:4819:2000:210:f3ff:fe03:14d0`, so hostnames are recommended. The output looks the same as usual, with the IPv6 address on the “interesting ports” line being the only IPv6 giveaway.]()

[While IPv6 hasn't exactly taken the world by storm, it gets significant use in some (usually Asian) countries and most modern operating systems support it. To use Nmap with IPv6, both the source and target of your scan must be configured for IPv6. If your ISP (like most of them) does not allocate IPv6 addresses to you, free tunnel brokers are widely available and work fine with Nmap. I use the free IPv6 tunnel broker]()[ service at ]()[`http://www.tunnelbroker.net`](http://www.tunnelbroker.net/). Other tunnel brokers are [listed at Wikipedia](http://en.wikipedia.org/wiki/List_of_IPv6_tunnel_brokers). 6to4 tunnels are another popular, free approach.

On Windows, raw-socket IPv6 scans are supported only on ethernet devices (not tunnels), and only on Windows Vista[ and later. Use the `--unprivileged`]()[ option in other situations.]()

[`-A` (Aggressive scan options) ]()[ ]()

[This option enables additional advanced and aggressive options. Presently this enables OS detection (`-O`), version scanning (`-sV`), script scanning (`-sC`) and traceroute (`--traceroute`).]()[ More features may be added in the future. The point is to enable a comprehensive set of scan options without people having to remember a large set of flags. However, because script scanning with the default set is considered intrusive, you should not use `-A` against target networks without permission. This option only enables features, and not timing options (such as `-T4`) or verbosity options (`-v`) that you might want as well. Options which require privileges (e.g. root access) such as OS detection and traceroute will only be enabled if those privileges are available.]()

[`--datadir *`<directoryname>`*` (Specify custom Nmap data file location) ]()[ ]()

[Nmap obtains some special data at runtime in files named `nmap-service-probes`, `nmap-services`, `nmap-protocols`, `nmap-rpc`, `nmap-mac-prefixes`, and `nmap-os-db`. If the location of any of these files has been specified (using the `--servicedb` or `--versiondb` options), that location is used for that file. After that, Nmap searches these files in the directory specified with the `--datadir` option (if any). Any files not found there, are searched for in the directory specified by the `NMAPDIR`]()[ environment variable. Next comes `~/.nmap`]()[ for real and effective UIDs; or on Windows, `*`<HOME>`*\AppData\Roaming\nmap` (where *`<HOME>`* is the user's home directory, like `C:\Users\user`). This is followed by the location of the `nmap` executable and the same location with `../share/nmap` appended. Then a compiled-in location such as `/usr/local/share/nmap` or `/usr/share/nmap`. ]()

[`--servicedb *`<services file>`*` (Specify custom services file) ]()[ ]()

[Asks Nmap to use the specified services file rather than the `nmap-services` data file that comes with Nmap. Using this option also causes a fast scan (`-F`) to be used. See the description for `--datadir` for more information on Nmap's data files.]()

[`--versiondb *`<service probes file>`*` (Specify custom service probes file) ]()[ ]()

[Asks Nmap to use the specified service probes file rather than the `nmap-service-probes` data file that comes with Nmap. See the description for `--datadir` for more information on Nmap's data files.]()

[`--send-eth` (Use raw ethernet sending) ]()[ ]()

[Asks Nmap to send packets at the raw ethernet (data link) layer rather than the higher IP (network) layer. By default, Nmap chooses the one which is generally best for the platform it is running on. Raw sockets (IP layer)]()[ are generally most efficient for Unix machines, while ethernet frames are required for Windows operation since Microsoft disabled raw socket support. Nmap still uses raw IP packets on Unix despite this option when there is no other choice (such as non-ethernet connections).]()

[`--send-ip` (Send at raw IP level) ]()[ ]()

[Asks Nmap to send packets via raw IP sockets rather than sending lower level ethernet frames. It is the complement to the `--send-eth` option discussed previously.]()

[`--privileged` (Assume that the user is fully privileged) ]()[ ]()

[Tells Nmap to simply assume that it is privileged enough to perform raw socket sends, packet sniffing, and similar operations that usually require root privileges]()[]()[ on Unix systems. By default Nmap quits if such operations are requested but `geteuid` is not zero. `--privileged` is useful with Linux kernel capabilities and similar systems that may be configured to allow unprivileged users to perform raw-packet scans. Be sure to provide this option flag before any flags for options that require privileges (SYN scan, OS detection, etc.). The `NMAP_PRIVILEGED`]()[ environment variable may be set as an equivalent alternative to `--privileged`.]()

[`--unprivileged` (Assume that the user lacks raw socket privileges) ]()[ ]()[ ]()

[This option is the opposite of `--privileged`. It tells Nmap to treat the user as lacking network raw socket and sniffing privileges. This is useful for testing, debugging, or when the raw network functionality of your operating system is somehow broken. The `NMAP_UNPRIVILEGED`]()[ environment variable may be set as an equivalent alternative to `--unprivileged`.]()

[`--release-memory` (Release memory before quitting) ]()[ ]()

[This option is only useful for memory-leak debugging. It causes Nmap to release allocated memory just before it quits so that actual memory leaks are easier to spot. Normally Nmap skips this as the OS does this anyway upon process termination.]()

[`-V`; `--version` (Print version number) ]()[ ]()[ ]()

[Prints the Nmap version number and exits.]()

[`-h`; `--help` (Print help summary page) ]()[ ]()[ ]()

[Prints a short help screen with the most common command flags. Running Nmap without any arguments does the same thing.]()

---

[Prev](https://nmap.org/book/man-output.html)Output

[Up](https://nmap.org/book/man.html)Chapter 15. Nmap Reference Guide

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/man-runtime-interaction.html)Runtime Interaction