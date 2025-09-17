---
title: "NSE Libraries | Nmap Network Scanning"
source_url: https://nmap.org/book/nse-library.html
fetched_at: 2025-09-17T16:44:02.385416+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 9. Nmap Scripting Engine](https://nmap.org/book/nse.html)
* NSE Libraries

[Prev](https://nmap.org/book/nse-scripts.html)

[Next](https://nmap.org/book/nse-api.html)

NSE Libraries
----------

[]()

In addition to the significant built-in capabilities of Lua, we have written or integrated many extension libraries which make script writing more powerful and convenient. These libraries (sometimes called modules) are compiled if necessary and installed along with Nmap. They have their own directory, `nselib`, which is installed in the configured Nmap data directory. Scripts need only [`require`](https://lua.org/manual/5.4/manual.html#pdf-require) the default libraries in order to use them.

### List of All Libraries ###

 This list is just an overview to give an idea of what libraries are available. Developers will want to consult the complete documentation at [`https://nmap.org/nsedoc/`](https://nmap.org/nsedoc/).

`afp` []()

[This library was written by Patrik Karlsson \<patrik@cqure.net\> to facilitate
communication with the Apple AFP Service. It is not feature complete and still missing several functions.]()

[`ajp` ]()[ ]()

[A basic AJP 1.3 implementation based on documentation available from Apache
mod\_proxy\_ajp; ]()[`http://httpd.apache.org/docs/2.2/mod/mod_proxy_ajp.html`](http://httpd.apache.org/docs/2.2/mod/mod_proxy_ajp.html)

`amqp` []()

[The AMQP library provides some basic functionality for retrieving information
about an AMQP server's properties.]()

[`asn1` ]()[ ]()

[ASN.1 functions.]()

[`base32` ]()[ ]()

[Base32 encoding and decoding. Follows RFC 4648.]()

[`base64` ]()[ ]()

[Base64 encoding and decoding. Follows RFC 4648.]()

[`bin` ]()[ ]()

[Pack and unpack binary data.]()

[`bit` ]()[ ]()

[Bitwise operations on integers.]()

[`bitcoin` ]()[ ]()

[This library implements a minimal subset of the BitCoin protocol
It currently supports the version handshake and processing Addr responses.]()

[`bittorrent` ]()[ ]()

[Bittorrent and DHT protocol library which enables users to read information from a torrent file, decode bencoded (bittorrent
encoded) buffers, find peers associated with a certain torrent and
retrieve nodes discovered during the search for peers.]()

[`bjnp` ]()[ ]()

[An implementation of the Canon BJNP protocol used to discover and query
Canon network printers and scanner devices.]()

[`brute` ]()[ ]()

[The brute library is an attempt to create a common framework for performing
password guessing against remote services.]()

[`cassandra` ]()[ ]()

[Library methods for handling Cassandra Thrift communication as client]()

[`citrixxml` ]()[ ]()

[This module was written by Patrik Karlsson and facilitates communication
with the Citrix XML Service. It is not feature complete and is missing several functions and parameters.]()

[`comm` ]()[ ]()

[Common communication functions for network discovery tasks like
banner grabbing and data exchange.]()

[`creds` ]()[ ]()

[The credential class stores found credentials in the Nmap registry]()

[`cvs` ]()[ ]()

[A minimal CVS (Concurrent Versions System) pserver protocol implementation which currently only supports authentication.]()

[`datafiles` ]()[ ]()

[Read and parse some of Nmap's data files: `nmap-protocols`,`nmap-rpc`, `nmap-services`, and `nmap-mac-prefixes`.]()

[`dhcp` ]()[ ]()

[Implement a Dynamic Host Configuration Protocol (DHCP) client.]()

[`dhcp6` ]()[ ]()

[Minimalistic DHCP6 (Dynamic Host Configuration Protocol for IPv6)
implementation supporting basic DHCP6 Solicit requests The library
is structured around the following classes:]()

* [DHCP6.Option	- DHCP6 options encoders (for requests) and decoders]()

[ (for responses)]()

* [DHCP6.Request	- DHCP6 request encoder and decoder]()
* [DHCP6.Response	- DHCP6 response encoder and decoder]()
* [Helper - The helper class, primary script interface]()

[]()

[`dns` ]()[ ]()

[Simple DNS library supporting packet creation, encoding, decoding,
and querying.]()

[`dnsbl` ]()[ ]()

[A minimalistic DNS BlackList library implemented to facilitate querying
various DNSBL services. The current list of services has been implemented
based on the following compilations of services:]()

* [`http://en.wikipedia.org/wiki/Comparison_of_DNS_blacklists`](http://en.wikipedia.org/wiki/Comparison_of_DNS_blacklists)
* [`http://www.robtex.com`](http://www.robtex.com/)
* [`http://www.sdsc.edu/~jeff/spam/cbc.html`](http://www.sdsc.edu/~jeff/spam/cbc.html)

`dnssd` []()

[Library for supporting DNS Service Discovery]()

[`drda` ]()[ ]()

[DRDA Library supporting a very limited subset of operations.]()

[`eap` ]()[ ]()

[EAP (Extensible Authentication Protocol) library supporting a
limited subset of features.]()

[`eigrp` ]()[ ]()

[A library supporting parsing and generating a limited subset of the Cisco' EIGRP packets.]()

[`ftp` ]()[ ]()

[FTP functions.]()

[`giop` ]()[ ]()

[GIOP Library supporting a very limited subset of operations]()

[`gps` ]()[ ]()

[A smallish gps parsing module.
Currently does GPRMC NMEA decoding]()

[`http` ]()[ ]()

[Implements the HTTP client protocol in a standard form that Nmap scripts can
take advantage of.]()

[`httpspider` ]()[ ]()

[A smallish httpspider library providing basic spidering capabilities
It consists of the following classes:]()

[`iax2` ]()[ ]()

[A minimalistic Asterisk IAX2 (Inter-Asterisk eXchange v2) VoIP protocol implementation.
The library implements the minimum needed to perform brute force password guessing.]()

[`imap` ]()[ ]()

[A library implementing a minor subset of the IMAP protocol, currently the
CAPABILITY, LOGIN and AUTHENTICATE functions. The library was initially
written by Brandon Enright and later extended and converted to OO-form by
Patrik Karlsson \<patrik@cqure.net\>]()

[`informix` ]()[ ]()

[Informix Library supporting a very limited subset of Informix operations]()

[`ipOps` ]()[ ]()

[Utility functions for manipulating and comparing IP addresses.]()

[`ipp` ]()[ ]()

[A small CUPS ipp (Internet Printing Protocol) library implementation]()

[`iscsi` ]()[ ]()

[An iSCSI library implementing written by Patrik Karlsson \<patrik@cqure.net\>
The library currently supports target discovery and login.]()

[`isns` ]()[ ]()

[A minimal Internet Storage Name Service (iSNS) implementation]()

[`jdwp` ]()[ ]()

[JDWP (Java Debug Wire Protocol) library implementing a set of commands needed to use remote debugging port and inject java bytecode.]()

[`json` ]()[ ]()

[Library methods for handling JSON data. It handles JSON encoding and
decoding according to RFC 4627.]()

[`ldap` ]()[ ]()

[Library methods for handling LDAP.]()

[`lfs` ]()[ ]()

[Returns a directory iterator listing the contents of the given path
Each time the iterator is called with dir\_obj it returns a directory entry's
name as a string, or nil if there are no more entries.]()

[`listop` ]()[ ]()

[Functional-style list operations.]()

[`match` ]()[ ]()

[Buffered network I/O helper functions.]()

[`membase` ]()[ ]()

[A smallish implementation of the Couchbase Membase TAP protocol
Based on the scarce documentation from the Couchbase Wiki:
x ]()[`http://www.couchbase.org/wiki/display/membase/SASL+Authentication+Example`](http://www.couchbase.org/wiki/display/membase/SASL+Authentication+Example)

`mobileme` []()

[A MobileMe web service client that allows discovering Apple devices
using the "find my iPhone" functionality.]()

[`mongodb` ]()[ ]()

[Library methods for handling MongoDB, creating and parsing packets.]()

[`msrpc` ]()[ ]()

[By making heavy use of the `smb` library, this library will call various MSRPC functions. The functions used here can be accessed over TCP ports 445 and 139, with an established session. A NULL session (the default) will work for some functions and operating systems (or configurations), but not for others.]()

[`msrpcperformance` ]()[ ]()

[This module is designed to parse the `PERF_DATA_BLOCK` structure, which is
stored in the registry under HKEY\_PERFORMANCE\_DATA. By querying this structure, you can
get a whole lot of information about what's going on.]()

[`msrpctypes` ]()[ ]()

[This module was written to marshall parameters for Microsoft RPC (MSRPC) calls. The values passed in and out are based
on structs defined by the protocol, and documented by Samba developers. For detailed breakdowns of the types, take a look at Samba 4.0's `.idl` files.]()

[`mssql` ]()[ ]()

[MSSQL Library supporting a very limited subset of operations.]()

[`mysql` ]()[ ]()

[Simple MySQL Library supporting a very limited subset of operations.]()

[`natpmp` ]()[ ]()

[This library implements the basics of NAT-PMP as described in the
NAT Port Mapping Protocol (NAT-PMP) draft: o ]()[`http://tools.ietf.org/html/draft-cheshire-nat-pmp-03`](http://tools.ietf.org/html/draft-cheshire-nat-pmp-03)

`ncp` []()

[A tiny implementation of the Netware Core Protocol (NCP).
While NCP was originally a Netware only protocol it's now present on
both Linux and Windows platforms running Novell eDirectory.]()

[`ndmp` ]()[ ]()

[A minimalistic NDMP (Network Data Management Protocol) library]()

[`netbios` ]()[ ]()

[Creates and parses NetBIOS traffic. The primary use for this is to send
NetBIOS name requests.]()

[`nmap` ]()[ ]()

[Interface with Nmap internals.]()

[`nrpc` ]()[ ]()

[A minimalistic library to support Domino RPC]()

[`nsedebug` ]()[ ]()

[Debugging functions for Nmap scripts.]()

[`omp2` ]()[ ]()

[This library was written to ease interaction with OpenVAS Manager servers
using OMP (OpenVAS Management Protocol) version 2.]()

[`openssl` ]()[ ]()

[OpenSSL bindings.]()

[`ospf` ]()[ ]()

[A minimalistic OSPF (Open Shortest Path First routing protocol) library, currently supporting IPv4 and the following
OSPF message types: HELLO]()

[`packet` ]()[ ]()

[Facilities for manipulating raw packets.]()

[`pcre` ]()[ ]()

[Perl Compatible Regular Expressions.]()

[`pgsql` ]()[ ]()

[PostgreSQL library supporting both version 2 and version 3 of the protocol.
The library currently contains the bare minimum to perform authentication.
Authentication is supported with or without SSL enabled and using the
plain-text or MD5 authentication mechanisms.]()

[`pop3` ]()[ ]()

[POP3 functions.]()

[`pppoe` ]()[ ]()

[A minimalistic PPPoE (Point-to-point protocol over Ethernet)
library, implementing basic support for PPPoE
Discovery and Configuration requests. The PPPoE protocol is ethernet based
and hence does not use any IPs or port numbers.]()

[`proxy` ]()[ ]()

[Functions for proxy testing.]()

[`rdp` ]()[ ]()

[A minimal RDP (Remote Desktop Protocol) library. Currently has functionality to determine encryption
and cipher support.]()

[`redis` ]()[ ]()

[A minimalistic Redis (in-memory key-value data store) library.]()

[`rmi` ]()[ ]()

[Library method for communicating over RMI (JRMP + java serialization)]()

[`rpc` ]()[ ]()

[RPC Library supporting a very limited subset of operations.]()

[`rpcap` ]()[ ]()

[This library implements the fundamentals needed to communicate with the
WinPcap Remote Capture Daemon. It currently supports authenticating to
the service using either NULL-, or Password-based authentication.
In addition it has the capabilities to list the interfaces that may be
used for sniffing.]()

[`rsync` ]()[ ]()

[A minimalist RSYNC (remote file sync) library]()

[`rtsp` ]()[ ]()

[This Real Time Streaming Protocol (RTSP) library implements only a minimal
subset of the protocol needed by the current scripts.]()

[`sasl` ]()[ ]()

[Simple Authentication and Security Layer (SASL).]()

[`shortport` ]()[ ]()

[Functions for building short portrules.]()

[`sip` ]()[ ]()

[A SIP library supporting a limited subset of SIP commands and methods]()

[`smb` ]()[ ]()

[Implements functionality related to Server Message Block (SMB, an extension of CIFS) traffic, which is a Windows protocol.]()

[`smbauth` ]()[ ]()

[This module takes care of the authentication used in SMB (LM, NTLM, LMv2, NTLMv2).]()

[`smtp` ]()[ ]()

[Simple Mail Transfer Protocol (SMTP) operations.]()

[`snmp` ]()[ ]()

[SNMP functions.]()

[`socks` ]()[ ]()

[A smallish SOCKS version 5 proxy protocol implementation]()

[`srvloc` ]()[ ]()

[A relatively small implementation of the Service Location Protocol.
It was initially designed to support requests for discovering Novell NCP
servers, but should work for any other service as well.]()

[`ssh1` ]()[ ]()

[Functions for the SSH-1 protocol. This module also contains functions for
formatting key fingerprints.]()

[`ssh2` ]()[ ]()

[Functions for the SSH-2 protocol.]()

[`sslcert` ]()[ ]()

[A library providing functions for collecting SSL certificates and storing
them in the host-based registry.]()

[`stdnse` ]()[ ]()

[Standard Nmap Scripting Engine functions. This module contains various handy
functions that are too small to justify modules of their own.]()

[`strbuf` ]()[ ]()

[String buffer facilities.]()

[`strict` ]()[ ]()

[Strict declared global library. Checks for undeclared global variables
during runtime execution.]()

[`stun` ]()[ ]()

[A library that implements the basics of the STUN protocol (Session
Traversal Utilities for NAT) per RFC3489 and RFC5389. A protocol
overview is available at ]()[`http://en.wikipedia.org/wiki/STUN`](http://en.wikipedia.org/wiki/STUN).

`tab` []()

[Arrange output into tables.]()

[`target` ]()[ ]()

[Utility functions to add new discovered targets to Nmap scan queue.]()

[`tftp` ]()[ ]()

[Library implementing a minimal TFTP server]()

[`tns` ]()[ ]()

[TNS Library supporting a very limited subset of Oracle operations]()

[`unpwdb` ]()[ ]()

[Username/password database library.]()

[`upnp` ]()[ ]()

[A UPNP library based on code from upnp-info initially written by Thomas Buchanan. The code was factored out from upnp-info and partly
re-written by Patrik Karlsson \<patrik@cqure.net\> in order to support
multicast requests.]()

[`url` ]()[ ]()

[URI parsing, composition, and relative URL resolution.]()

[`versant` ]()[ ]()

[A tiny library allowing some basic information enumeration from
Versant object database software (see]()[`http://en.wikipedia.org/wiki/Versant_Corporation`](http://en.wikipedia.org/wiki/Versant_Corporation)). The code is
entirely based on packet dumps captured when using the Versant
Management Center administration application.

`vnc` []()

[The VNC library provides some basic functionality needed in order to
communicate with VNC servers, and derivates such as Tight- or Ultra-
VNC.]()

[`vulns` ]()[ ]()

[Functions for vulnerability management.]()

[`vuzedht` ]()[ ]()

[A Vuze DHT protocol implementation based on the following documentation:
o ]()[`http://wiki.vuze.com/w/Distributed_hash_table`](http://wiki.vuze.com/w/Distributed_hash_table)

`wsdd` []()

[A library that enables scripts to send Web Service Dynamic Discovery probes
and perform some very basic decoding of responses. The library is in no way
a full WSDD implementation it's rather the result of some packet captures
and some creative coding.]()

[`xdmcp` ]()[ ]()

[Implementation of the XDMCP (X Display Manager Control Protocol) based on: x ]()[`http://www.xfree86.org/current/xdmcp.pdf`](http://www.xfree86.org/current/xdmcp.pdf)

`xmpp` []()

[A XMPP (Jabber) library, implementing a minimal subset of the protocol
enough to do authentication brute-force.]()

### [Hacking NSE Libraries]() ###

[ A common mistake when editing libraries is to accidentally use a global variable instead of a local one. Different libraries using the same global variable can be the cause of mysterious bugs. Lua's scope assignment is global by default, so this mistake is easy to make. ]()

[ To help correct this problem, NSE uses a library adapted from the standard Lua distribution called `strict.lua`.]()[]() The library will raise a runtime error on any access or modification of a global variable which was undeclared in the file scope. A global variable is considered declared if the library makes an assignment to the global name (even `nil`) in the file scope.

### Adding C Modules to Nselib ###

[]()

 A few of the modules included in nselib are written in C or C++ rather than Lua. Two examples are `bit` and `pcre`. We recommend that modules be written in Lua if possible, but C and C++ may be more appropriate if performance is critical or (as with the `pcre` and `openssl` modules) you are linking to an existing C library. This section describes how to write your own compiled extensions to nselib.

 The Lua C API is described at length in [*Programming in Lua, Second Edition*](http://www.amazon.com/dp/8590379825?tag=secbks-20), so this is a short summary. C modules consist of functions that follow the protocol of the [lua\_CFunction](https://lua.org/manual/5.4/manual.html#lua_CFunction) type. The functions are registered with Lua and assembled into a library by calling the `luaL_newlib`[]() function. A special initialization function provides the interface between the module and the rest of the NSE code. By convention the initialization function is named in the form `luaopen_*`<module>`*`.

 The most straightforward compiled module that comes with NSE is `openssl`.[]() This module serves as a good example for a beginning module writer. The source code for `openssl` source is in `nse_openssl.cc` and `nse_openssl.h`. Most of the other compiled modules follow this `nse_*`<module name>`*.cc` naming convention.

 Reviewing the `openssl` module shows that one of the functions in `nse_openssl.cc` is `l_md5`, which calculates an MD5 digest. Its function prototype is:

```
static int l_md5(lua_State *L);

```

The prototype shows that `l_md5` matches the lua\_CFunction type. The function is static because it does not have to be visible to other compiled code. Only an address is required to register it with Lua. Later in the file, `l_md5` is entered into an array of type luaL\_Reg and associated with the name `md5`:

```
static const struct luaL_Reg openssllib[] = {
  { "md5", l_md5 },
  { NULL, NULL }
};

```

This function will now be known as `md5` to NSE. Next the library is registered with a call to `luaL_newlib` inside the initialization function `luaopen_openssl`, as shown next. Some lines relating to the registration of OpenSSL BIGNUM types have been omitted:

```
LUALIB_API int luaopen_openssl(lua_State *L) {
  luaL_newlib(L, openssllib);
  return 1;
}

```

The function `luaopen_openssl` is the only function in the file that is exposed in `nse_openssl.h`. `OPENSSLLIBNAME` is simply the string `"openssl"`.

 After a compiled module is written, it must be added to NSE by including it in the list of standard libraries in `nse_main.cc`. Then the module's source file names must be added to `Makefile.in` in the appropriate places. For both these tasks you can simply follow the example of the other C modules. For the Windows build, the new source files must be added to the `mswin32/nmap.vcproj` project file using MS Visual Studio (see [the section called “Compile from Source Code”](https://nmap.org/book/inst-windows.html#inst-win-source)).

---

[Prev](https://nmap.org/book/nse-scripts.html)NSE Scripts

[Up](https://nmap.org/book/nse.html)Chapter 9. Nmap Scripting Engine

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nse-api.html)Nmap API