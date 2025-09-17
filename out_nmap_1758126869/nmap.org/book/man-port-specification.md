---
title: "Port Specification and Scan Order | Nmap Network Scanning"
source_url: https://nmap.org/book/man-port-specification.html
fetched_at: 2025-09-17T16:35:47.303723+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 15. Nmap Reference Guide](https://nmap.org/book/man.html)
* Port Specification and Scan Order

[Prev](https://nmap.org/book/man-port-scanning-techniques.html)

[Next](https://nmap.org/book/man-version-detection.html)

Port Specification and Scan Order
----------

[]()

In addition to all of the scan methods discussed previously, Nmap offers options for specifying which ports are scanned and whether the scan order is randomized or sequential. By default, Nmap scans the most common 1,000 ports for each protocol. []()

`-p *`<port ranges>`*` (Only scan specified ports) []()

[This option specifies which ports you want to scan and overrides the default. Individual port numbers are OK, as are ranges separated by a hyphen (e.g. `1-1023`). The beginning and/or end values of a range may be omitted, causing Nmap to use 1 and 65535, respectively. So you can specify `-p-` to scan ports from 1 through 65535. Scanning port zero]()[ is allowed if you specify it explicitly. For IP protocol scanning (`-sO`), this option specifies the protocol numbers you wish to scan for (0–255).]()

[When scanning a combination of protocols (e.g. TCP and UDP), you can specify a particular protocol by preceding the port numbers by `T:` for TCP, `U:` for UDP, `S:` for SCTP, or `P:` for IP Protocol. The qualifier lasts until you specify another qualifier. For example, the argument `-p U:53,111,137,T:21-25,80,139,8080` would scan UDP ports 53, 111,and 137, as well as the listed TCP ports. Note that to scan both UDP and TCP, you have to specify `-sU` and at least one TCP scan type (such as `-sS`, `-sF`, or `-sT`). If no protocol qualifier is given, the port numbers are added to all protocol lists.]()

[]()[

 Ports can also be specified by name according to what the port is referred to in the `nmap-services`. You can even use the wildcards `*` and `?` with the names. For example, to scan FTP and all ports whose names begin with “http”, use `-p ftp,http*`. Be careful about shell expansions and quote the argument to `-p` if unsure.

Ranges of ports can be surrounded by square brackets to indicate ports inside that range that appear in `nmap-services`. For example, the following will scan all ports in `nmap-services` equal to or below 1024: `-p [-1024]`. Be careful with shell expansions and quote the argument to `-p` if unsure.

]()[ `--exclude-ports *`<port ranges>`*` (Exclude the specified ports from scanning) ]()[ ]()

[This option specifies which ports you do want Nmap to exclude from scanning. The *`<port ranges>`* are specified similar to `-p`. For IP protocol scanning (`-sO`), this option specifies the protocol numbers you wish to exclude (0–255).]()

[When ports are asked to be excluded, they are excluded from all types of scans (i.e. they will not be scanned under any circumstances). This also includes the discovery phase.]()

[`-F` (Fast (limited port) scan) ]()[ ]()[ ]()

[Specifies that you wish to scan fewer ports than the default. Normally Nmap scans the most common 1,000 ports for each scanned protocol. With `-F`, this is reduced to 100.]()

[Nmap needs an `nmap-services` file with frequency information in order to know which ports are the most common (see ]()[the section called “Well Known Port List: `nmap-services`”](https://nmap.org/book/nmap-services.html) for more about port frequencies). If port frequency information isn't available, perhaps because of the use of a custom `nmap-services` file, Nmap scans all named ports plus ports 1-1024. In that case, `-F` means to scan only ports that are named in the services file.

`-r` (Don't randomize ports) []()[ ]()

[By default, Nmap randomizes the scanned port order (except that certain commonly accessible ports are moved near the beginning for efficiency reasons). This randomization is normally desirable, but you can specify `-r` for sequential (sorted from lowest to highest) port scanning instead.]()

[`--port-ratio *`<ratio>`*<decimal number between 0 and 1>` ]()[

Scans all ports in `nmap-services` file with a ratio greater than the one given. *`<ratio>`* must be between 0.0 and 1.0.

]()[ `--top-ports *`<n>`*` ]()[

Scans the *`<n>`* highest-ratio ports found in `nmap-services` file after excluding all ports specified by `--exclude-ports`. *`<n>`* must be 1 or greater.

]()

---

[Prev](https://nmap.org/book/man-port-scanning-techniques.html)Port Scanning Techniques

[Up](https://nmap.org/book/man.html)Chapter 15. Nmap Reference Guide

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/man-version-detection.html)Service and Version Detection