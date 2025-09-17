---
title: "SunRPC Numbers: nmap-rpc | Nmap Network Scanning"
source_url: https://nmap.org/book/nmap-rpc.html
fetched_at: 2025-09-17T16:48:32.853961+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 14. Understanding and Customizing Nmap Data Files](https://nmap.org/book/data-files.html)
* SunRPC Numbers: nmap-rpc

[Prev](https://nmap.org/book/nmap-service-probes.html)

[Next](https://nmap.org/book/nmap-os-db.html)

SunRPC Numbers: `nmap-rpc`
----------

[]()

As with `nmap-services`, `nmap-rpc` simply maps numbers to names. In this case, SunRPC program numbers are mapped to the program name which uses them. [Example 14.3](https://nmap.org/book/nmap-rpc.html#data-files-nmap-rpc-file) offers a typical
excerpt.

Example 14.3. Excerpt from `nmap-rpc`

[]()

```
rpcbind         100000  portmap sunrpc rpcbind
rstatd          100001  rstat rup perfmeter rstat_svc
rusersd         100002  rusers
nfs             100003  nfsprog nfsd
ypserv          100004  ypprog
mountd          100005  mount showmount
rpc.operd       100080  opermsg         # Sun Online-Backup
# DMFE/DAWS (Defense Automated Warning System)
#
Gqsrv           200034  gqsrv
Ppt             200035  ppt
Pmt             200036  pmt

```

Nmap only cares about the first two whitespace-separated columns—the program name and number. It doesn't look at any aliases or comments that may appear beyond
that.[]()Blank lines and those starting with pound comments are permitted. This format is the same as used by `/etc/rpc` on Unix, so administrators may use that file instead if they desire.

`nmap-rpc` is only used by the
RPC grinding[]()feature of Nmap version descriptions. That feature is covered in[the section called “RPC Grinding”](https://nmap.org/book/vscan-post-processors.html#version-detection-rpc).

Users rarely change `nmap-rpc`. When they
do, it is usually to add a custom service or a public one that is
missing from the latest `nmap-rpc`. In the latter
case, please send a note to me at `<[fyodor@nmap.org](mailto:fyodor@nmap.org)>` so that I can
add it to the next version. As with`nmap-services`, some administrators strip the file down,
removing obscure RPC programs to save scan time. The same warning
applies: specify your stripped `nmap-rpc` with the`--datadir` option[]()rather than installing it where it will be used implicitly.

---

[Prev](https://nmap.org/book/nmap-service-probes.html)Version Scanning DB: nmap-service-probes

[Up](https://nmap.org/book/data-files.html)Chapter 14. Understanding and Customizing Nmap Data Files

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nmap-os-db.html)Nmap OS Detection DB: nmap-os-db