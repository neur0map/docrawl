---
title: "Linux/Unix Compilation and Installation from Source Code | Nmap Network Scanning"
source_url: https://nmap.org/book/inst-source.html
fetched_at: 2025-09-17T16:37:02.306592+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap](https://nmap.org/book/install.html)
* Linux/Unix Compilation and Installation from Source Code

[Prev](https://nmap.org/book/install.html)

[Next](https://nmap.org/book/inst-linux.html)

Linux/Unix Compilation and Installation from Source Code
----------

[]()[]()[]()[]()[]()

[]()While binary packages
(discussed in later sections) are available for most platforms, compilation and
installation from source code is the traditional and most powerful way
to install Nmap. This ensures that the
latest version is available and allows Nmap
to adapt to the library availability and directory structure of your
system. For example, Nmap uses the
OpenSSL cryptography libraries for version detection when
available, but most binary packages do not include this
functionality. On the other hand, binary packages are generally
quicker and easier to install, and allow for consistent management
(installation, removal, upgrading, etc.) of all packaged software on the system.

Source installation is usually a painless process—the build system is designed to auto-detect as much as possible. Here are the steps required for a default install:

1. Download the latest version of Nmap in .tar.bz2 (bzip2 compression) or .tgz (gzip compression) format from [`https://nmap.org/download.html`](https://nmap.org/download.html).

2. Decompress the downloaded tarball with a command such as:

   **bzip2 -cd nmap-*`<VERSION>`*.tar.bz2 | tar xvf -**

   With GNU tar, the simpler command **tar xvjf
   nmap-*`<VERSION>`*.tar.bz2** does the
   trick. If you downloaded the .tgz version, replacebzip2 with gzipin the decompression command.

3. Change into the newly created directory: **cd nmap-*`<VERSION>`***

4. Configure the build system: **./configure**

   If the configuration succeeds, an ASCII art dragon appears to congratulate you on successful configuration and warn you to be careful, as shown in [Example 2.7](https://nmap.org/book/inst-source.html#ex-configure-success).

   Example 2.7. Successful configuration screen

   ```
   flog~/nmap> ./configure
   checking build system type... x86_64-unknown-linux-gnu
   [hundreds of lines cut]
   configure: creating ./config.status
   config.status: creating Makefile
   config.status: creating nsock_config.h
   config.status: nsock_config.h is unchanged
      (  )   /\   _                 (
       \ |  (  \ ( \.(               )                      _____
     \  \ \  `  `   ) \             (  ___                 / _   \
    (_`    \+   . x  ( .\            \/   \____-----------/ (o)   \_
   - .-               \+  ;          (  O                           \____
   (__                +- .( -'.- <.   \_____________  `              \  /
   (_____            ._._: <_ - <- _- _  VVVVVVV VV V\                \/
     .    /./.+-  . .- /  +--  - .    (--_AAAAAAA__A_/                |
     (__ ' /x  / x _/ (                \______________//_              \_______
    , x / ( '  . / .  /                                  \___'          \     /
       /  /  _/ /    +                                       |           \   /
      '  (__/                                               /              \/
                                                          /                  \
                NMAP IS A POWERFUL TOOL -- USE CAREFULLY AND RESPONSIBLY
   Configuration complete.  Type make (or gmake on some *BSD machines) to compile.

   ```

5. Build Nmap (and the Zenmap GUI if its requirements are met):**make**

   Note that GNU Make is required. On BSD-derived Unix systems,
   this is often installed as *gmake*. So if**make** returns a bunch of errors such as“`Makefile, line 1: Need an
   operator`”, try running **gmake** instead.

6. Become a privileged user for system-wide install: **su root**

   This step may be skipped if you only have an unprivileged shell
   account on the system. In that case, you will likely need to pass
   the `--prefix` option to `configure`in step four as described in the next section.

7. Install Nmap, support files, docs, etc.: **make install**

   Congratulations! Nmap is now installed as `/usr/local/bin/nmap`! Run it with no arguments for a quick help screen.

As you can see above, a simple source compilation and install
consists of little more than running **./configure;make;make
install** as root. However, there are a number of options available to configure that affect the way Nmap is built.

### Configure Directives ###

[]()

Most of the Unix build options are controlled by the `configure` script, as used in step number four above. There are dozens of command-line parameters and environmental variables which affect the way Nmap is built. Run **./configure --help** for a huge list with brief descriptions. These are not applicable to building Nmap on Windows. Here are the options which are either specific to Nmap or particularly important:

`--prefix=*`<directoryname>`*`

This option, which is standard to theconfigure scripts of most software,
determines where Nmap and its components
are installed. By default, the prefix is`/usr/local`, meaning thatnmap is installed in `/usr/local/bin`, the
man page (`nmap.1`) is installed in`/usr/local/man/man1`, and the data files
(`nmap-os-db`,`nmap-services`, `nmap-service-probes`,
etc.) are installed under `/usr/local/share/nmap`.
If you only wish to change the path of certain components, use the
options `--bindir`, `--datadir`, and/or`--mandir`. An example usage of`--prefix` would be to install
Nmap in my account as an unprivileged user.
I would run **./configure --prefix=*`</home/fyodor>`***. Nmap creates subdirectories like `/home/fyodor/man/man1` in the install stage if they do not already exist.

`--without-zenmap`[

This option prevents the Zenmap graphical frontend from being installed. Normally the build system checks your system for requirements such as the Python scripting language and then installs Zenmap if they are all available.

]()[`--with-openssl=*`<directoryname>`*`]()[

The version detection system and Nmap Scripting Engine are able to probe SSL-encrypted services using the free OpenSSL libraries. Normally the Nmap build system looks for these libraries on your system and include this capability if they are found. If they are in a location your compiler does not search for by default, but you still want them to be used, specify `--with-openssl=*`<directoryname>`*`. Nmap then looks in *`<directoryname>`*/libs for the OpenSSL libraries themselves and *`<directoryname>`*/include for the necessary header files. Specify `--without-openssl` to disable SSL entirely.

]()[]()

[Some distributions ship with user OpenSSL libraries that allow running programs, but not the developer files needed to compile them. Without these developer packages, Nmap will not have OpenSSL support. On Debian-based systems]()[, install the `libssl-dev` package.]()[ On Red Hat–based systems,]()[ install `openssl-devel`.]()[]()

[`--with-libpcap=*`<directoryname>`*`]()

[Nmap uses the ]()[Libpcap library](http://www.tcpdump.org/) for capturing raw IP packets. Nmap normally looks for an existing copy of Libpcap on your system and uses that if the version number and platform is appropriate. Otherwise Nmap includes its own recent copy of Libpcap (with some local modifications described in `libpcap/NMAP_MODIFICATIONS` in the Nmap source directory). If you wish to force Nmap to link with your own Libpcap, pass the option `--with-libpcap=*`<directoryname>`*` to configure. Nmap then expects the Libpcap library to be in `*`<directoryname>`*/lib/libpcap.a` and the include files to be in `*`<directoryname>`*/include`. Nmap will always use the version of Libpcap included in its tarball if you specify `--with-libpcap=included`.

`--with-libpcre=*`<directoryname>`*`

PCRE is a Perl-compatible regular expression library available from [`http://www.pcre.org`](http://www.pcre.org/). Nmap normally looks for a copy on your system, and then falls back to its own copy if that fails. If your PCRE library is not in your compiler's standard search path, Nmap probably will not find it. In that case you can tell Nmap where it can be found by specifying the option `--with-libpcre=*`<directoryname>`*` to configure. Nmap then expects the library files to be in `*`<directoryname>`*/lib` and the include files to be in `*`<directoryname>`*/include`. In some cases, you may wish to use the PCRE libraries included with Nmap in preference to those already on your system. In that case, specify `--with-libpcre=included`.

`--with-libdnet=*`<directoryname>`*`

Libdnet is an excellent networking library that Nmap uses for sending raw ethernet frames. The version in the Nmap tree is heavily modified (particularly the Windows code), so the default is to use that included version. If you wish to use a version already installed on your system instead, specify `--with-libdnet=*`<directoryname>`*`. Nmap then expects the library files to be in `*`<directoryname>`*/lib` and the include files to be in `*`<directoryname>`*/include`.

`--with-localdirs`

This simple option tells Nmap to look in `/usr/local/lib` and `/usr/local/include` for important library and header files. This should never be necessary, except that some people put such libraries in `/usr/local` without configuring their compiler to find them. If you are one of those people, use this option.

### Environment Variables ###

[]()

The `configure` script is sensitive to several environment variables. These are some of those variables and their effects.

[]()[]()[`CFLAGS`, ]()[`CXXFLAGS`, LDFLAGS]()

[Extra options to pass to the C compiler, C++ compiler, and linker, respectively. Because parts of Nmap are written in C and others in C++, it's best to use both `CFLAGS` and `CXXFLAGS` if you're going to use one of them.]()

[]()[]()[]()[`LINGUAS`]()

[By default, **make install** will install all the available translations of the Nmap man page in addition to the English one. The `LINGUAS` environment variable can control which translations are installed. Its value should be a space-separated list of ISO language codes. For example, to install only the French and German translations, you might run **LINGUAS="fr de" make install**. To disable the installation of all translations, run configure with the `--disable-nls` option or set `LINGUAS` to the empty string.]()

### [If You Encounter Compilation Problems]() ###

[]()

In an ideal world, software would always compile perfectly (and quickly) on every system. Unfortunately, society has not yet reached that state of nirvana. Despite all our efforts to make Nmap portable, compilation issues occasionally arise. Here are some suggestions in case the source distribution compilation fails.

Upgrade to the latest Nmap

Check [`https://nmap.org/download.html`](https://nmap.org/download.html) to make sure you are using the latest version of Nmap. The problem may have already been fixed.

Read the error message carefully

Scroll up in the output screen and examine the error
messages given when commands fail. It is often best to find the first
error message, as that often causes a cascade of further errors. Read
the error message carefully, as it could indicate a system problem
such as low disk space or a broken compiler. Users with programming
skills may be able to resolve a wider range of problems themselves.
If you make code changes to fix the problem, please send a patch
(created with **diff -uw *`<oldfile>`* *`<newfile>`***) and any details about your problem and platform to *nmap-dev* as described in [the section called “Bugs”](https://nmap.org/book/man-bugs.html). Integrating the change into the base Nmap distribution allows many other users to benefit, and prevents you from having to make the changes with each new Nmap version.

Ask Google and other Internet resources

Try searching for the exact error message on Google or other search engines. You might also want to browse recent activity on the Nmap development
(*nmap-dev*)[list—archives and a search interface are available at ]()[`https://seclists.org`](https://seclists.org/).

Ask *nmap-dev*

If none of your research leads to a solution, try
sending a report to the Nmap development
(*nmap-dev*) mailing list, as described in[the section called “Bugs”](https://nmap.org/book/man-bugs.html).

Consider binary packages[

Binary packages of Nmap are available on most platforms and are
usually easy to install. The downsides are that they may not be as
up-to-date and you lose some of the flexibility of self-compilation.
Later sections of this chapter describe how to find binary packages on
many platforms, and even more are available via Internet searching.
Obviously you should only install binary packages from reputable
sources.

]()

---

[Prev](https://nmap.org/book/install.html)Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap

[Up](https://nmap.org/book/install.html)Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/inst-linux.html)Linux Distributions