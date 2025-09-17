---
title: "Using Customized Data Files | Nmap Network Scanning"
source_url: https://nmap.org/book/data-files-replacing-data-files.html
fetched_at: 2025-09-17T16:35:36.307319+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 14. Understanding and Customizing Nmap Data Files](https://nmap.org/book/data-files.html)
* Using Customized Data Files

[Prev](https://nmap.org/book/data-files-nse.html)

[Next](https://nmap.org/book/man.html)

Using Customized Data Files
----------

[]()

Any or all of the Nmap data files may be replaced with versions
customized to the user's liking. They can only be replaced in whole—you cannot specify changes that will be merged with the original
files at runtime. When Nmap looks for each file, it searches by name
in many directories and selects the first one found. This is the
analogous to the way your Unix shell finds programs you ask to execute
by searching through the directories in your`PATH`[]()one
at a time in order. The following list gives the Nmap directory
search order. It shows that an `nmap-services`found in the directory specified by `--datadir`will be
used in preference to one found in `~/.nmap/`because the former is searched first.

Nmap data file directory search order[]()

1. If the `--datadir` option was specified, check the directory given as its argument.[]()

2. If the `NMAPDIR` environmental variable[]() is set, check that directory.

3. If Nmap is not running on Windows, search in `~/.nmap`[]() of the user running Nmap. It tries the real user ID's home directory, and then the effective UID's if they differ.

4. Check the directory in which the Nmap binary resides. On non-Windows platforms, additionally check the same directory with `../share/nmap` appended.

5. []()

   Check the compiled-in `NMAPDATADIR` directory. That value is defined to `c:\nmap` on Windows, and `*`<$prefix>`*/share/nmap` on Unix. *`<$prefix>`* is `/usr/local` for the default source build and `/usr` for the Linux RPMs. The *`<$prefix>`* can be changed by giving **./configure** the `--prefix` option when compiling the source.

Nmap does not check for files in the current working directory
(`.`) for
the same security reasons that `.` should not
appear first on your shell execution `PATH`. On a
shared system, a malicious user could place bogus data files in a
shared directory such as `/tmp`. Those files
could be malformed, causing Nmap to complain and exit, or they could
cause Nmap to skip important ports. If Nmap tried`.`, other users who happened to run Nmap
in that shared directory would get the bogus versions. This could
also happen by accident if you inadvertently ran Nmap in a directory
that happened to have a file named`nmap-services` (or one of the other ones).
Users who really want Nmap to try the current directory early may
set the environment variable `NMAPDIR` to`.` at their own risk.

This list shows the many choices users have when deciding how to
replace a file with their own customized version. The option I
usually recommend is to place the customized files in a special
directory named appropriately for the change. For example, an`nmap-services` stripped to contain just the
hundred most common ports could be placed in`~/nmap-fewports`. Then specify this directory
with the `--datadir` option. This ensures that the
customized files are only used intentionally. Since the Nmap
output-to-file formats include the Nmap command-line used, you will
know which files were used when reviewing the logs later.

Another option is to simply edit the original in `NMAPDATADIR`. This is rarely recommended, as the edited file will
likely be overwritten the next time Nmap is upgraded. Additionally,
this makes it hard to use the original files if you suspect that your
replacements are causing a problem. This also makes it difficult to
compare your version with the original to recall what you changed.

A third option is to place the customized files in your Unix`~/.nmap` directory. Of course you should only
insert files that you have changed. The others will still be
retrieved from `NMAPDATADIR` as usual. This is very convenient, as Nmap
will use the customized files implicitly whenever you run it. That
can be a disadvantage as well. Users sometimes forget the files
exist. When they upgrade Nmap to a version with newer data files, the
old copies in `~/.nmap` will still be used,
reducing the quality of results.

Setting the `NMAPDIR`environment variable to the directory with files
is another alternative. This can be useful when testing a new version
of Nmap. Suppose you obtain Nmap version 5.21, notice the huge list
of changes, and decide to test it out before replacing your current
known-working version. You might compile it in`~/src/nmap-5.21`, but execute it there and Nmap
tries to read the data files from`/usr/local/share/nmap`. Those are the old
versions, since Nmap 5.21 has not yet been installed. Simply set`NMAPDIR` to `~/src/nmap-5.21`, test
to your heart's content, and then perform the **make
install**. A disadvantage to using `NMAPDIR`regularly is that the directory name is not recorded in Nmap output
files like it is when `--datadir` is used
instead.

[]()

---

[Prev](https://nmap.org/book/data-files-nse.html)Files Related to Scripting

[Up](https://nmap.org/book/data-files.html)Chapter 14. Understanding and Customizing Nmap Data Files

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/man.html)Chapter 15. Nmap Reference Guide