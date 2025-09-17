---
title: "Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap | Nmap Network Scanning"
source_url: https://nmap.org/book/install.html
fetched_at: 2025-09-17T16:35:24.060355+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap

[Prev](https://nmap.org/book/history-future.html)

[Next](https://nmap.org/book/inst-source.html)

Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap
==========

Table of Contents

* [Introduction](https://nmap.org/book/install.html#inst-intro)
  * [Testing Whether Nmap is Already Installed](https://nmap.org/book/install.html#inst-already)
  * [Command-line and Graphical Interfaces](https://nmap.org/book/install.html#inst-interfaces)
  * [Downloading Nmap](https://nmap.org/book/install.html#inst-download)
  * [Verifying the Integrity of Nmap Downloads](https://nmap.org/book/install.html#inst-integrity)
  * [Obtaining Nmap from the Subversion (SVN) Repository](https://nmap.org/book/install.html#inst-svn)

* [Linux/Unix Compilation and Installation from Source Code](https://nmap.org/book/inst-source.html)
  * [Configure Directives](https://nmap.org/book/inst-source.html#inst-configure)
  * [Environment Variables](https://nmap.org/book/inst-source.html#inst-env)
  * [If You Encounter Compilation Problems](https://nmap.org/book/inst-source.html#inst-compilation-probs)

* [Linux Distributions](https://nmap.org/book/inst-linux.html)
  * [RPM-based Distributions (Red Hat, Mandrake, SUSE, Fedora)](https://nmap.org/book/inst-linux.html#inst-rpm)
  * [Updating Red Hat, Fedora, Mandrake, and Yellow Dog Linux with Yum](https://nmap.org/book/inst-linux.html#inst-yum)
  * [Debian Linux and Derivatives such as Ubuntu](https://nmap.org/book/inst-linux.html#inst-debian)
  * [Other Linux Distributions](https://nmap.org/book/inst-linux.html#inst-linux-other)

* [Windows](https://nmap.org/book/inst-windows.html)
  * [Windows Self-installer](https://nmap.org/book/inst-windows.html#inst-win-exe)
  * [Command-line Zip Binaries](https://nmap.org/book/inst-windows.html#inst-win-zip)
    * [Installing the Nmap zip binaries](https://nmap.org/book/inst-windows.html#inst-win-zip-install)

  * [Compile from Source Code](https://nmap.org/book/inst-windows.html#inst-win-source)
  * [Executing Nmap on Windows](https://nmap.org/book/inst-windows.html#inst-win-exec)

* [Apple Mac OS X](https://nmap.org/book/inst-macosx.html)
  * [Executable Installer](https://nmap.org/book/inst-macosx.html#inst-macosx-installer)
  * [Compile from Source Code](https://nmap.org/book/inst-macosx.html#inst-macosx-source)
    * [Compile Nmap from source code](https://nmap.org/book/inst-macosx.html#idm45818756601264)
    * [Compile Zenmap from source code](https://nmap.org/book/inst-macosx.html#idm45818756589824)

  * [Third-party Packages](https://nmap.org/book/inst-macosx.html#inst-macosx-third-party)
  * [Executing Nmap on Mac OS X](https://nmap.org/book/inst-macosx.html#inst-macosx-exec)

* [Other Platforms (BSD, Solaris, AIX, AmigaOS)](https://nmap.org/book/inst-other-platforms.html)
  * [FreeBSD / OpenBSD / NetBSD](https://nmap.org/book/inst-other-platforms.html#inst-bsd)
    * [OpenBSD Binary Packages and Source Ports Instructions](https://nmap.org/book/inst-other-platforms.html#inst-openbsd)
    * [FreeBSD Binary Package and Source Ports Instructions](https://nmap.org/book/inst-other-platforms.html#inst-freebsd)
    * [NetBSD Binary Package Instructions](https://nmap.org/book/inst-other-platforms.html#inst-netbsd)

  * [Oracle/Sun Solaris](https://nmap.org/book/inst-other-platforms.html#inst-solaris)
  * [IBM AIX](https://nmap.org/book/inst-other-platforms.html#inst-aix)
  * [AmigaOS](https://nmap.org/book/inst-other-platforms.html#inst-amiga)
  * [Other proprietary UNIX (HP-UX, IRIX, etc.)](https://nmap.org/book/inst-other-platforms.html#inst-unix)

* [Removing Nmap](https://nmap.org/book/inst-removing-nmap.html)

[]()

Introduction
----------

Nmap can often be installed or upgraded with a single command,
so don't let the length of this chapter scare you. Most readers will
use the table of contents to skip
directly to sections that concern them. This chapter describes how to install
Nmap on many platforms, including both source
code compilation and binary installation methods. Graphical and
command-line versions of Nmap are described and contrasted.
Nmap removal instructions are also provided in case you change your
mind.

### Testing Whether Nmap is Already Installed ###

[]()

The first step toward obtaining Nmap is to check whether you already
have it. Many free operating system distributions (including most
Linux and BSD systems) come with Nmap packages, although they may not be
installed by default. On Unix systems, open a terminal window and try executing the command**nmap `--version`**.
If Nmap exists and is in your`PATH`,[]()you should see output similar to that in [Example 2.1](https://nmap.org/book/install.html#ex-checking-for-nmap).

[]()

Example 2.1. Checking for Nmap and determining its version number

[]()

```
felix~> nmap --version

Nmap version 4.76 ( https://nmap.org )
felix~>

```

If Nmap does *not*exist on the system (or if your `PATH` is incorrectly
set), an error message such as`nmap: Command not found` is reported. As the
example above shows, Nmap responds to the command by printing its
version number (here `4.76`).

Even if your system already has a copy of Nmap, you should
consider upgrading to the latest version available from [`https://nmap.org/download.html`](https://nmap.org/download.html).[]()Newer versions often run faster, fix important bugs, and feature
updated operating system and service version detection databases. A
list of changes since the version already on your system can be found
at [`https://nmap.org/changelog.html`](https://nmap.org/changelog.html).[]()Nmap output examples in this document may not match the output produced by
older versions.

### Command-line and Graphical Interfaces ###

Nmap has traditionally been a command-line tool run from
a Unix shell or (more recently) Windows command prompt. This allows
experts to quickly execute a command that does exactly what they want
without having to maneuver through a bunch of configuration panels and
scattered option fields. This also makes Nmap easier to script and
enables easy sharing of useful commands among the user
community.

One downside of the command-line approach is that it can be
intimidating for new and infrequent users.
Nmap offers more than a hundred
command-line options, although many are obscure features or debugging
controls that most users can ignore. Many graphical frontends have been
created for those users who prefer a GUI interface. Nmap has traditionally included a simple GUI for Unix named NmapFE[](), but that was replaced in 2007 by Zenmap,
which we have been developing since 2005. Zenmap is far more powerful and effective than NmapFE, particularly in results viewing. Zenmap's tab-based interface lets you search and sort
results, and also browse them in several ways (host details, raw Nmap
output, and ports/hosts). It works on Linux, Windows, Mac
OS X, and other platforms. Zenmap is covered in depth in [Chapter 12, *Zenmap GUI Users' Guide*](https://nmap.org/book/zenmap.html). The rest of this book focuses on command-line Nmap invocations.
Once you understand how the
command-line options work and can interpret the output, using Zenmap or
the other available Nmap GUIs is easy. Nmap's options work the same way
whether you choose them from radio buttons and menus or type them at a
command-line.

### Downloading Nmap ###

[]()

Nmap.Org is the official source for downloading Nmap source
code and binaries for Nmap and Zenmap. Source code is distributed in
bzip2 and gzip compressed tar files, and binaries are available for
Linux (RPM format), Windows (NSIS executable installer) and Mac OS X (`.dmg` disk image). Find all of this at [`https://nmap.org/download.html`](https://nmap.org/download.html).

### Verifying the Integrity of Nmap Downloads ###

[]()

It often pays to be paranoid about the
integrity of files downloaded from the Internet. Popular packages
such as Sendmail ([example](http://cert.org/advisories/CA-2002-28.html)),
OpenSSH ([example](http://cert.org/advisories/CA-2002-24.html)),
tcpdump, Libpcap, BitchX, Fragrouter, and many others have been
infected with malicious trojans. Software distributions sites at the
Free Software Foundation, Debian, and SourceForge have also been
successfully compromised. This has never happened to Nmap, but one
should always be careful. To verify the authenticity of an Nmap
release, consult the PGP detached signatures or cryptographic hashes
(including SHA1 and MD5) posted for the release in the Nmap signatures
directory at [`https://nmap.org/dist/sigs/?C=M&O=D`](https://nmap.org/dist/sigs/?C=M&O=D).

The most secure verification mechanism is detached PGP[]()signatures. As the signing key is never stored on production servers,
even someone who successfully compromises the web server couldn't
forge and properly sign a trojan release. While numerous applications
are able to verify PGP signatures, I recommend [GNU Privacy Guard (GPG)](http://www.gnupg.org/).

[]()

Nmap releases are signed with a special
Nmap Project Signing Key,[]()which can be obtained from the major keyservers or [`https://svn.nmap.org/nmap/docs/nmap_gpgkeys.txt`](https://svn.nmap.org/nmap/docs/nmap_gpgkeys.txt). My key is
included in that file too. The keys can be imported with the command**gpg --import nmap\_gpgkeys.txt**. You only need to do
this once, then you can verify all future Nmap releases from that
machine. Before trusting the
keys, verify that the fingerprints match the values shown in [Example 2.2](https://nmap.org/book/install.html#ex-check-gpg-keys).

Example 2.2. Verifying the Nmap and Fyodor PGP Key Fingerprints

```
flog~> gpg --fingerprint nmap fyodor
pub 1024D/33599B5F 2005-04-24
    Key fingerprint = BB61 D057 C0D7 DCEF E730 996C 1AF6 EC50 3359 9B5F
uid                Fyodor <fyodor@insecure.org>
sub 2048g/D3C2241C 2005-04-24

pub 1024D/6B9355D0 2005-04-24
    Key fingerprint = 436D 66AB 9A79 8425 FDA0 E3F8 01AF 9F03 6B93 55D0
uid                Nmap Project Signing Key (https://insecure.org/)
sub 2048g/A50A6A94 2005-04-24

```

For every Nmap package download file
(e.g. `nmap-4.76.tar.bz2` and`nmap-4.76-setup.exe`), there is a corresponding
file in the `sigs` directory with `.asc` appended
to the name (e.g. `nmap-4.76.tar.bz2.asc`).
This is the detached signature file.

With the proper PGP key in your keyring and the detached
signature file downloaded, verifying an Nmap release takes a single
GPG command, as shown in [Example 2.3](https://nmap.org/book/install.html#ex-gpg-verify-nmap-release-good). That example assumes that the verified file can be found in the same directory by simply removing “.asc” from the signature filename. When that isn't the case, simply pass the target filename as the final argument to GPG. If the file has been
tampered with, the results will look like [Example 2.4](https://nmap.org/book/install.html#ex-gpg-verify-nmap-release-bad).

Example 2.3. Verifying PGP key fingerprints (Successful)

```
flog> gpg --verify nmap-4.76.tar.bz2.asc
gpg: Signature made Fri 12 Sep 2008 02:03:59 AM PDT using DSA key ID 6B9355D0
gpg: Good signature from "Nmap Project Signing Key (http://www.insecure.org/)"

```

Example 2.4. Detecting a bogus file

```
flog> gpg --verify nmap-4.76.tar.bz2.asc nmap-4.76-hacked.tar.bz2
gpg: Signature made Fri 12 Sep 2008 02:03:59 AM PDT using DSA key ID 6B9355D0
gpg: BAD signature from "Nmap Project Signing Key (http://www.insecure.org/)"

```

While PGP signatures are the recommended validation technique,
SHA2, SHA1, and MD5 (among other)
hashes[]()[]()are made available for more casual
validation. An attacker who can manipulate your Internet traffic in
real time (and is extremely skilled) or who compromises Nmap.Org
and replaces both the distribution file and digest file, could defeat
this test. However, it can be useful to check the authoritative
Nmap.Org hashes if you obtain Nmap from a third party or feel it
might have been accidentally corrupted. For every Nmap package download
file, there is a corresponding file in the `sigs` directory with`.digest.txt` appended to the name
(e.g. `nmap-4.76.tar.bz2.digest.txt`). An example
is shown in [Example 2.5](https://nmap.org/book/install.html#ex-digest-file). This is the detached
signature file. The hashes from the digest file can be verified using common tools such asgpg, sha1sum, or md5sum, as shown in [Example 2.6, “Verifying Nmap hashes”](https://nmap.org/book/install.html#ex-digest-file-verify).

Example 2.5. A typical Nmap release digest file

```
flog> cat sigs/nmap-4.76.tgz.digest.txt
nmap-4.76.tgz:    MD5 = 54 B5 C9 E3 F4 4C 1A DD  E1 7D F6 81 70 EB 7C FE
nmap-4.76.tgz:   SHA1 = 4374 CF9C A882 2C28 5DE9  D00E 8F67 06D0 BCFA A403
nmap-4.76.tgz: RMD160 = AE7B 80EF 4CE6 DBAA 6E65  76F9 CA38 4A22 3B89 BD3A
nmap-4.76.tgz: SHA224 = 524D479E 717D98D0 2FB0A42B 9A4E6E52 4027C9B6 1D843F95
                        D419F87F
nmap-4.76.tgz: SHA256 = 0E960E05 53EB7647 0C8517A0 038092A3 969DB65C BE23C03F
                        D6DAEF1A CDCC9658
nmap-4.76.tgz: SHA384 = D52917FD 9EE6EE62 F5F456BF E245675D B6EEEBC5 0A287B27
                        3CAA4F50 B171DC23 FE7808A8 C5E3A49A 4A78ACBE A5AEED33
nmap-4.76.tgz: SHA512 = 826CD89F 7930A765 C9FE9B41 1DAFD113 2C883857 2A3A9503
                        E4C1E690 20A37FC8 37564DC3 45FF0C97 EF45ABE6 6CEA49FF
                        E262B403 A52F4ECE C23333A0 48DEDA66

```

Example 2.6. Verifying Nmap hashes

```
flog> gpg --print-md sha256 nmap-4.76.tgz
nmap-4.76.tgz: 0E960E05 53EB7647 0C8517A0 038092A3 969DB65C BE23C03F D6DAEF1A
               CDCC9658
flog> sha1sum nmap-4.76.tgz
4374cf9ca8822c285de9d00e8f6706d0bcfaa403  nmap-4.76.tgz
flog> md5sum nmap-4.76.tgz
54b5c9e3f44c1adde17df68170eb7cfe  nmap-4.76.tgz

```

While releases from Nmap.Org are signed as described in this
section, certain Nmap add-ons, interfaces, and platform-specific
binaries are developed and distributed by other parties. They have
different mechanisms for establishing the authenticity of their
downloads.

### Obtaining Nmap from the Subversion (SVN) Repository ###

[]()[]()

In addition to regular stable and development releases, the
latest Nmap source code is always available using the [Subversion (SVN) revision control
system](http://subversion.apache.org/). This delivers new features and version/OS detection
database updates immediately as they are developed. The downside is that
SVN head revisions aren't always as stable as official releases. So
SVN is most useful for Nmap developers and users who need a fix which
hasn't yet been formally released.

SVN write access is strictly limited to top Nmap
developers, but everyone has read access to the repository. Check out
the latest code using the command **svn co
https://svn.nmap.org/nmap**. Then you can later
update your source code by typing **svn up** in your
working directory.

While most users only follow the `/nmap`directory in SVN, there is one other
interesting directory: `/nmap-exp`. This directory
contains *experimental* Nmap branches which Nmap
developers create when they wish to try new things without
destabilizing Nmap proper. When developers feel that an experimental
branch is ready for wider-scale testing, they will generally email the
location to the *nmap-dev* mailing list.

Once Nmap is checked out, you can build it from source code just as you would with the Nmap tarball (described later in this chapter).

If you would like real-time (or digested) notification and diffs by email when any
changes are made to Nmap, sign up for the nmap-svn mailing
list at [`https://nmap.org/mailman/listinfo/svn`](https://nmap.org/mailman/listinfo/svn).

[]()

---

[Prev](https://nmap.org/book/history-future.html)The History and Future of Nmap

[Up](https://nmap.org/book/toc.html)Nmap Network Scanning

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/inst-source.html)Linux/Unix Compilation and Installation from Source Code