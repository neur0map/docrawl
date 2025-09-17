---
title: "Other Platforms (BSD, Solaris, AIX, AmigaOS) | Nmap Network Scanning"
source_url: https://nmap.org/book/inst-other-platforms.html
fetched_at: 2025-09-17T16:37:20.837830+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap](https://nmap.org/book/install.html)
* Other Platforms (BSD, Solaris, AIX, AmigaOS)

[Prev](https://nmap.org/book/inst-macosx.html)

[Next](https://nmap.org/book/inst-removing-nmap.html)

Other Platforms (BSD, Solaris, AIX, AmigaOS)
----------

Most Nmap users run the software on Linux, Windows, or Mac OS X. We consider those our top priority platforms and we maintain build and test machines to ensure that each build supports them well.

Nmap also runs on many other platforms that we don't have the resources to personally test or build binaries packages for as frequently. We rely on a passionate user community to help Nmap maintain top-notch support for the platforms on this page, and we're always happy to see Nmap expand onto other platforms.

The following sections provide tips for running Nmap on specific platforms.

### FreeBSD / OpenBSD / NetBSD ###

[]()[]()[]()[]()

The BSD flavors are well supported by Nmap, so you can simply
compile it from source as described in [the section called “Linux/Unix Compilation and Installation from Source Code”](https://nmap.org/book/inst-source.html). This provides the normal advantages of always having the latest
version and a flexible build process. If you prefer binary packages,
these \*BSD variants each maintain their own Nmap packages. Many BSD
systems also have a *ports tree* which standardizes the compilation of
popular applications. Instructions for installing Nmap on
the most popular \*BSD variants follow.

#### OpenBSD Binary Packages and Source Ports Instructions ####

[]()

According to the [OpenBSD FAQ](http://www.openbsd.org/faq/), users“are HIGHLY advised to use packages over building an application
from ports. The OpenBSD ports team considers packages to be the goal
of their porting work, not the ports themselves.” That same
FAQ contains detailed instructions for each method. Here is a summary:

Installation using binary packages

1. Choose a mirror from [`http://www.openbsd.org/ftp.html`](http://www.openbsd.org/ftp.html), then FTP in and grab the Nmap
   package from`/pub/OpenBSD/*`<version>`*/packages/*`<platform>`*/nmap-*`<version>`*.tgz`.
   Or obtain it from the OpenBSD distribution CD-ROM.

2. As root, execute: **pkg\_add -v nmap-*`<version>`*.tgz**

Installation using the source ports tree

1. If you do not already have a copy of the ports tree, obtain it via CVS using instructions at [`http://openbsd.org/faq/faq15.html`](http://openbsd.org/faq/faq15.html).

2. As root, execute the following command (replace `/usr/ports` with your local ports directory if it differs):

   **cd /usr/ports/net/nmap && make install clean**

#### FreeBSD Binary Package and Source Ports Instructions ####

[]()

The FreeBSD project has a whole [chapter](http://www.freebsd.org/doc/en_US.ISO8859-1/books/handbook/ports.html)in their Handbook describing the package and port installation
processes. A brief summary of the process follows.

##### Installation of the binary package #####

The easiest way to install the binary Nmap package is to run**pkg\_add -r nmap**. You can then run the same command
with the `zenmap` argument if you want the X-Window
front-end. If you wish to obtain the package manually instead,
retrieve it from [`http://freshports.org/security/nmap`](http://freshports.org/security/nmap) and [`http://freshports.org/security/zenmap`](http://freshports.org/security/zenmap) or
the CDROM and run **pkg\_add *`<packagename.tgz>`***.

##### Installation using the source ports tree #####

1. The ports tree is often installed with the system
   itself (usually in `/usr/ports`). If you do not already have it,
   specific installation instructions are provided in the FreeBSD Handbook
   chapter referenced above.

2. As root, execute the following command (replace `/usr/ports` with your local ports directory if it differs):

   **cd /usr/ports/security/nmap && make install clean**

#### NetBSD Binary Package Instructions ####

[]()

NetBSD has packaged Nmap for an enormous number of platforms, from the normal i386 to PlayStation 2, PowerPC, VAX, SPARC, MIPS, Amiga, ARM, and several platforms that I have never even heard of! A list of NetBSD Nmap packages is available from [`ftp://ftp.netbsd.org/pub/NetBSD/packages/pkgsrc/net/nmap/README.html`](ftp://ftp.netbsd.org/pub/NetBSD/packages/pkgsrc/net/nmap/README.html) and a description of using their package system to install applications is available at [`http://netbsd.org/Documentation/pkgsrc/using.html`](http://netbsd.org/Documentation/pkgsrc/using.html).

### Oracle/Sun Solaris ###

[]()[]()[]()

Solaris has long been well-supported by Nmap, though we rely
heavily on the Nmap community to help keep it that way. We recommend
compiling and installing Nmap from source as described in [the section called “Linux/Unix Compilation and Installation from Source Code”](https://nmap.org/book/inst-source.html). If you have trouble, try sending a report
with full details to the *nmap-dev* mailing
list[](), as described in [the section called “Bugs”](https://nmap.org/book/man-bugs.html). Also let us know if you develop a patch which
improves Solaris support so we can incorporate it into
Nmap for the benefit of other Solaris users.

### IBM AIX ###

[]()[]()

Nmap can be installed from source on IBM AIX by following the instructions in[the section called “Linux/Unix Compilation and Installation from Source Code”](https://nmap.org/book/inst-source.html). You need only pay attention to a few
details.

You must use the**gcc**[]() compiler,
not **xlc**. Nmap's configure script will automatically find**gcc** if it is somewhere in the`PATH`[]()environment variable.

Some editions of the default**as**[]()assembler either[crash](http://gcc.gnu.org/bugzilla/show_bug.cgi?id=33577) or[produce object
files that can't be linked](http://gcc.gnu.org/bugzilla/show_bug.cgi?id=46072). This is what's happening if you see compiler
output like this:

```
g++: internal compiler error: Segmentation fault (program as)
Please submit a full bug report,
with preprocessed source if appropriate.
See <http://gcc.gnu.org/bugs.html> for instructions.

```

```
ld: 0711-596 SEVERE ERROR: Object ../nsock/src/libnsock.a[nsock_core.o]
        An RLD for section 2 (.data) refers to symbol 1794,
        but the storage class of the symbol is not C_EXT or C_HIDEXT.

```

You can work around this problem by installing the **as** from
GNU binutils[](). (But not**ld**; you want to continue using the default**ld**.) These instructions were tested on AIX 7.1 with`binutils-2.22` from`http://ftp.gnu.org/gnu/binutils`.

```
$ bzip2 -dc binutils-2.22.tar.bz2 | tar -xvf -
$ cd binutils-2.22
$ ./configure --disable-werror --disable-largefile CFLAGS="-O2 -Wall"
$ gmake
$ cd gas
$ su
# gmake install

```

This will install **as** in `/usr/local/bin`.
The custom`CFLAGS`[]()omit `-g`,[]()which would otherwise cause one of the **as** errors you are
trying to work around. You must make sure that`/usr/local/bin` appears before`/usr/bin` in`PATH`[]()while building and configuring Nmap.

```
$ export PATH="/usr/local/bin:$PATH"

```

In some cases GCC is configured to use an absolute path to the
assembler. In this case you will have to temporarily move the default
assembler out of the way. You can test whether this is the case by
passing the `-print-prog-name=as` option to**gcc**:

```
$ gcc -print-prog-name=as
/usr/bin/as

```

If you see the output `/usr/bin/as`, then
you must disable the system `as` with a command like**mv /usr/bin/as /usr/bin/as.backup**. If you see the
output `as`, then no other changes should
be required.

Now follow the instructions in [the section called “Linux/Unix Compilation and Installation from Source Code”](https://nmap.org/book/inst-source.html).

### AmigaOS ###

[]()

One of the wonders of open source development is that resources
are often directed towards what people find exciting rather than having
an exclusive focus on profits as most corporations do. It is along
those lines that the Amiga port came about.
Diego Casorran[]() performed
most of the work and sent in a clean patch which was integrated into
the main Nmap distribution. In general, AmigaOS users should be able
to simply follow the source compilation instructions in [the section called “Linux/Unix Compilation and Installation from Source Code”](https://nmap.org/book/inst-source.html). You may encounter a few hurdles on some
systems, but I presume that must be part of the fun for Amiga
fanatics.

### Other proprietary UNIX (HP-UX, IRIX, etc.) ###

[]()[]()

Nmap has in the past supported many proprietary Unix flavors such as HP-UX and
SGI IRIX. We depend heavily on the user community to
maintain adequate support for these systems. If you have trouble, try
sending a report with full details to the *nmap-dev* mailing list[](), as described in [the section called “Bugs”](https://nmap.org/book/man-bugs.html). Also let us know if
you develop a patch which
improves support on your platform so we can incorporate it into Nmap.

---

[Prev](https://nmap.org/book/inst-macosx.html)Apple Mac OS X

[Up](https://nmap.org/book/install.html)Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/inst-removing-nmap.html)Removing Nmap