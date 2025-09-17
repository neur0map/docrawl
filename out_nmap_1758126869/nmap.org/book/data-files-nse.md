---
title: "Files Related to Scripting | Nmap Network Scanning"
source_url: https://nmap.org/book/data-files-nse.html
fetched_at: 2025-09-17T16:48:40.840462+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 14. Understanding and Customizing Nmap Data Files](https://nmap.org/book/data-files.html)
* Files Related to Scripting

[Prev](https://nmap.org/book/nmap-protocols.html)

[Next](https://nmap.org/book/data-files-replacing-data-files.html)

Files Related to Scripting
----------

The scripts used by the Nmap Scripting Engine may be considered
another kind of data file. Scripts are stored in a`scripts` subdirectory of one of the directories
listed in[the section called “Using Customized Data Files”](https://nmap.org/book/data-files-replacing-data-files.html).[]()The name of
each script file ends in`.nse`.[]()For all the details on scripts see [Chapter 9, *Nmap Scripting Engine*](https://nmap.org/book/nse.html).

All of the files in the script directory are executable scripts,
except for one:`script.db`[]().
This file is a plain-text cache of which categories each script belongs
to. It should not be edited directly; use the`--script-updatedb`[]()option instead.

NSE's extension modules[]()(see[the section called “NSE Libraries”](https://nmap.org/book/nse-library.html)) are stored
in the `nselib` subdirectory of
the Nmap data directory, normally the same one`scripts` is in. This is where modules like`shortport` and `stdnse` are kept, in
files whose names end in`.lua`.[]()

---

[Prev](https://nmap.org/book/nmap-protocols.html)IP Protocol Number List: nmap-protocols

[Up](https://nmap.org/book/data-files.html)Chapter 14. Understanding and Customizing Nmap Data Files

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/data-files-replacing-data-files.html)Using Customized Data Files