---
title: "Linux Distributions | Nmap Network Scanning"
source_url: https://nmap.org/book/inst-linux.html
fetched_at: 2025-09-17T16:36:43.306098+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap](https://nmap.org/book/install.html)
* Linux Distributions

[Prev](https://nmap.org/book/inst-source.html)

[Next](https://nmap.org/book/inst-windows.html)

Linux Distributions
----------

Linux is the most popular platform for running Nmap. In one user
survey, 86% said that Linux was at least one of the platforms on which
they run Nmap. The first release of Nmap in
1997 *only* ran on Linux.

Linux users can choose between a source code install or using
binary packages provided by their distribution or Insecure.Org. The
binary packages are generally quicker and easier to install, and are
often slightly customized to use the distribution's standard directory
paths and such. These packages also allow for consistent management
in terms of upgrading, removing, or surveying software on the system.
A downside is that packages created by the distributions are
necessarily behind the Nmap.Org source releases. Most Linux
distributions keep their Nmap package
relatively current, though a few are way out of date. Choosing the
source install allows for more flexibility in determining how Nmap is
built and optimized for your system. To build Nmap from source, see[the section called “Linux/Unix Compilation and Installation from Source Code”](https://nmap.org/book/inst-source.html). Here are simple package instructions
for the most common distributions.

### RPM-based Distributions (Red Hat, Mandrake, SUSE, Fedora) ###

[]()[]()[]()[]()[]()[]()

I build RPM packages for every release of Nmap and post them to
the Nmap download page at [`https://nmap.org/download.html`](https://nmap.org/download.html).
I build two packages: The `nmap` package contains
just the command-line executable and data files, while the`zenmap` package contains the optional Zenmapgraphical frontend (see [Chapter 12, *Zenmap GUI Users' Guide*](https://nmap.org/book/zenmap.html)).
The `zenmap` package requires
that the `nmap` package be installed first.

[]()Installing via RPM is quite easy—it
even downloads the package for you when given the proper URLs. The following example downloads and installs Nmap 4.68, including the frontend. Of course you should use the latest version at the download site above instead. Any existing RPM-installed versions are
upgraded. [Example 2.8](https://nmap.org/book/inst-linux.html#ex-nmap-install-from-rpms) demonstrates this installation process.

Example 2.8. Installing Nmap from binary RPMs

```
# rpm -vhU https://nmap.org/dist/nmap-4.68-1.i386.rpm
Retrieving https://nmap.org/dist/nmap-4.68-1.i386.rpm
Preparing...                ########################################### [100%]
   1:nmap                   ########################################### [100%]
# rpm -vhU https://nmap.org/dist/zenmap-4.68-1.noarch.rpm
Retrieving https://nmap.org/dist/zenmap-4.68-1.noarch.rpm
Preparing...                ########################################### [100%]
   1:zenmap                 ########################################### [100%]

```

As the filenames above imply, these binary RPMs were created for normal PCs (x86 architecture).[]() I also distribute x86\_64[]() binaries for 64-bit Linux users. These binaries won't work for the relatively few Linux users on other platforms such as SPARC, Alpha, or PowerPC. They also may refuse to install if your library versions are sufficiently different from what the RPMs were initially built on. One option in these cases would be to find binary RPMs prepared by your Linux vendor for your specific distribution. The original install CDs or DVD are a good place to start. Unfortunately, those may not be current or available. Another option is to install Nmap from source code as described previously, though you lose the binary package maintenance consistency benefits. A third option is to build and install your own binary RPMs from the source RPMs distributed from the download page above. [Example 2.9](https://nmap.org/book/inst-linux.html#ex-nmap-install-from-srpms) demonstrates this technique with Nmap 4.68.

Example 2.9. Building and installing Nmap from source RPMs

```
> rpmbuild --rebuild https://nmap.org/dist/nmap-4.68-1.src.rpm
[ hundreds of lines cut ]
Wrote: /home/fyodor/rpmdir/RPMS/i386/nmap-4.68-1.i386.rpm
[ cut ]
> su
Password:
# rpm -vhU /home/fyodor/rpmdir/RPMS/i386/nmap-4.68-1.i386.rpm
Preparing...                ########################################### [100%]
   1:nmap                   ########################################### [100%]
#

```

It is not necessary to rebuild Zenmap in this fashion because the
Zenmap RPM is architecture-independent (“noarch”). For that
reason there are no Zenmap source RPMs.

Removing RPM packages is as easy as **rpm -e nmap zenmap**.

### Updating Red Hat, Fedora, Mandrake, and Yellow Dog Linux with Yum ###

[]()[]()[]()[]()[]()[]()

The Red Hat, Fedora, Mandrake, and Yellow Dog Linux
distributions have an application named Yumwhich manages software installation and updates from central RPM
repositories. This makes software installation and updates trivial.
Since distribution-specific Yum repositories are normally used, you
know the software has already been tested for compatibility with your
particular distribution. Most distributions do maintain Nmap in their
Yum repository, but they don't always keep it up to date. This is
particularly problematic if you (like most people) don't always
quickly update to the latest release of your distribution. If you are
running a two-year old Linux release, Yum will often give you a
two-year-old version of Nmap. Even the latest version of
distributions often take months to update to a new Nmap release. So
for the latest version of Nmap on these systems, try the RPMs we
distribute as described in the previous section. But if our
RPMs aren't compatible with your system or you are in a great hurry,
installing Nmap from Yum is usually as simple as executing**yum install nmap** (run **yum install nmap zenmap**if you would like the GUI too, though some distributions don't yet package Zenmap). Yum takes care of contacting a
repository on the Internet, finding the appropriate package for
your architecture, and then installing it along with any necessary
dependencies. This is shown (edited for brevity) in [Example 2.10](https://nmap.org/book/inst-linux.html#ex-nmap-install-from-yum). You can later perform**yum update** to install available updates to Nmap and other
packages in the repository.

Example 2.10. Installing Nmap from a system Yum repository

```
flog~# yum install nmap
Setting up Install Process
Parsing package install arguments
Resolving Dependencies
--> Running transaction check
---> Package nmap.x86_64 2:4.52-1.fc8 set to be updated
--> Finished Dependency Resolution
Dependencies Resolved
=============================================================================
 Package                 Arch       Version          Repository        Size
=============================================================================
Installing:
 nmap                    x86_64     2:4.52-1.fc8     updates           1.0 M

Transaction Summary
=============================================================================
Install      1 Package(s)         
Update       0 Package(s)         
Remove       0 Package(s)         

Total download size: 1.0 M
Is this ok [y/N]: y
Downloading Packages:
(1/1): nmap-4.52-1.fc8.x8 100% |=========================| 1.0 MB    00:02     
Running Transaction Test
Transaction Test Succeeded
Running Transaction
  Installing: nmap                         ######################### [1/1]

Installed: nmap.x86_64 2:4.52-1.fc8
Complete!

```

### Debian Linux and Derivatives such as Ubuntu ###

[]()[]()[]()

LaMont Jones[]()maintaining the Nmap `deb`packages, including keeping them reasonably up-to-date. The proper
upgrade/install command is **apt-get install nmap**.[]()This works for Debian derivatives such as Ubuntu too. Information on
the latest Debian “stable” Nmap package is available at [`http://packages.debian.org/stable/nmap`](http://packages.debian.org/stable/nmap) and the
development (“unstable”) Nmap and Zenmap packages are available from [`http://packages.debian.org/unstable/nmap`](http://packages.debian.org/unstable/nmap) and [`http://packages.debian.org/unstable/zenmap`](http://packages.debian.org/unstable/zenmap).

Sometimes Debian's Nmap releases are a year or more behind the current Nmap version. One option for obtaining the latest release is to compile from source code, as described in [the section called “Linux/Unix Compilation and Installation from Source Code”](https://nmap.org/book/inst-source.html). Another option is to download the RPM-format binaries from the Nmap download page, convert them to `deb` packages using the alien command, and then install them using dpkg, as described in the following list:

Steps for converting Nmap RPM files to Debian/Ubuntu `deb` format for installation on Debian/Ubuntu

1. If you don't have the alien command, install it with a command such as **sudo apt-get install alien**

2. Download the Nmap RPMs for your platform (x86 or x86-64) from [`https://nmap.org/download.html`](https://nmap.org/download.html). This description will use `nmap-5.21-1.x86_64.rpm`

3. Verify the download integrity as described in [the section called “Verifying the Integrity of Nmap Downloads”](https://nmap.org/book/install.html#inst-integrity).

4. Generate a Debian package with a command such as **sudo alien nmap-5.21-1.x86\_64.rpm**

5. Install the Debian package with a command such as **sudo dpkg --install nmap\_5.21-2\_amd64.deb**

6. Steps 2–5 can be repeated for the other Nmap RPMs such as Zenmap, Ncat, and Nping.

### Other Linux Distributions ###

There are far too many Linux distributions available to list
here, but even many of the obscure ones include Nmap in their package
tree. If they don't, you can simply compile from source code as
described in [the section called “Linux/Unix Compilation and Installation from Source Code”](https://nmap.org/book/inst-source.html).

---

[Prev](https://nmap.org/book/inst-source.html)Linux/Unix Compilation and Installation from Source Code

[Up](https://nmap.org/book/install.html)Chapter 2. Obtaining, Compiling, Installing, and Removing Nmap

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/inst-windows.html)Windows