---
title: "Removing Nmap | Nmap Network Scanning"
source_url: https://nmap.org/book/inst-removing-nmap.html
fetched_at: 2025-09-17T16:38:55.345860+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap](https://nmap.org/book/install.html)
* Removing Nmap

[Prev](https://nmap.org/book/inst-other-platforms.html)

[Next](https://nmap.org/book/host-discovery.html)

Removing Nmap
----------

[]()[]()

If your purpose for removing Nmap is
simply to upgrade to the latest version, you can usually use the
upgrade option provided by most binary package managers. Similarly,
installing the latest source code (as described in [the section called “Linux/Unix Compilation and Installation from Source Code”](https://nmap.org/book/inst-source.html)) generally overwrites any
previous from-source installations. Removing Nmap is a good idea if
you are changing install methods (such as from source to RPM or vice
versa) or if you are not using Nmap anymore and you care about the few
megabytes of disk space it consumes.

How to remove Nmap depends on how
you installed it initially (see previous sections). Ease of removal (and other maintenance) is a major advantage of most binary packages. For example, when Nmap is installed using
the RPM[]()system common on Linux distributions, it can be removed by
running the command **rpm -e nmap
zenmap** as root. Analogous options are offered by
most other package managers—consult their documentation for further
information.

If you installed Nmap from the Windows installer, simply open the Control Panel, select “Add or Remove Programs” and select the “Remove” button for Nmap. You can also remove Npcap unless you need it for other applications such as Wireshark.

If you installed Nmap from source
code, removal is slightly more difficult. If you still have the build
directory available (where you initially ran **make
install**), you can remove Nmap by
running **make uninstall**. If you no longer have that
build directory, type **nmap -V** to
obtain the Nmap version number. Then
download that source tarball for that version of
Nmap from [`https://nmap.org/dist/`](https://nmap.org/dist/) or [`https://nmap.org/dist-old/`](https://nmap.org/dist-old/).
Uncompress the tarball and change into the newly created directory
(`nmap-*`<version>`*`). Run**./configure**, including any install-path options that you specified
the first time (such as `--prefix` or`--datadir`). Then run **make
uninstall**. Alternatively, you can simply delete all the
Nmap-related files. If you used a default source install of Nmap
versions 4.50 or higher, the following commands remove it.

```
# cd /usr/local
# rm -f bin/nmap bin/nmapfe bin/xnmap
# rm -f man/man1/nmap.1 man/man1/zenmap.1
# rm -rf share/nmap
# ./bin/uninstall_zenmap

```

You may have to adjust the above commands slightly if you
specified `--prefix` or other install-path option when
first installing Nmap. The files relating to zenmap, nmapfe, and xnmap do not exist if you did not install the Zenmap frontend.

---

[Prev](https://nmap.org/book/inst-other-platforms.html)Other Platforms (BSD, Solaris, AIX, AmigaOS)

[Up](https://nmap.org/book/install.html)Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/host-discovery.html)Chapter 3. Host Discovery (“Ping Scanning”)