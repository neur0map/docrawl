---
title: "Files Used by Zenmap | Nmap Network Scanning"
source_url: https://nmap.org/book/zenmap-files.html
fetched_at: 2025-09-17T16:46:41.893442+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 12. Zenmap GUI Users' Guide](https://nmap.org/book/zenmap.html)
* Files Used by Zenmap

[Prev](https://nmap.org/book/zenmap-lang.html)

[Next](https://nmap.org/book/zenmap-conf.html)

Files Used by Zenmap
----------

[]()

 Zenmap uses a number of configuration and control files, and of course requires Nmap to be installed. Where the files are stored depends on the platform and how Zenmap was configured. The configuration files are divided into two categories: system files and per-user files.

### The `nmap` Executable ###

 Zenmap depends on the `nmap` command-line executable being installed. The program is first searched for in all of the directories specified in the `PATH`[]() environment variable.

 On some platforms the `nmap` command isn't commonly installed in any of the directories in `PATH`. As a convenience for those platforms, the following additional directories will be searched if the command is not found in the `PATH`:[]()

* On Mac OS X, the directory `/usr/local/bin` is searched.

* On Windows, the directory containing the Zenmap executable is searched.

 To use an absolute path to the executable, or if the executable is installed under a name other than `nmap`, modify the `nmap_command_path`[]() variable in the `[paths]` section of `zenmap.conf`.[]() For example, if you have installed `nmap` in `/opt/bin`, use

```
[paths]
nmap_command_path = /opt/bin/nmap

```

 Or if you have a custom-compiled version of Nmap called `nmap-custom`, use

```
[paths]
nmap_command_path = nmap-custom

```

 See [the section called “Description of `zenmap.conf`”](https://nmap.org/book/zenmap-conf.html).

### System Configuration Files ###

 These files affect the installation of Zenmap across an entire installation. On Unix and Mac OS X, they are in `*`<prefix>`*/share/zenmap`, where *`<prefix>`* is the filesystem prefix Zenmap was compiled with. The prefix is likely `/usr` or `/usr/local`, so Zenmap's file are probably in `/usr/share/zenmap` or `/usr/local/share/zenmap`. On Windows, the location also depends on where Zenmap was installed. They are probably in `C:\Program Files\Nmap\share\zenmap`. The Zenmap system configuration directory contains the following:

`config/`

 The files under `config` are copied to per-user configuration directories. See [the section called “Per-user Configuration Files”](https://nmap.org/book/zenmap-files.html#zenmap-user-conf).

`docs/`

 The files in the `docs` subdirectory are Zenmap's documentation files.

`locale/`

 The files in the `locale/` subdirectory contain translations of the text used by Zenmap into other languages.

`misc/profile_editor.xml`

 This file defines what options are presented by the profile editor (see [the section called “The Profile Editor”](https://nmap.org/book/zenmap-profile-editor.html)). It can be edited with care to alter the profile editor system-wide.

### Per-user Configuration Files ###

 These files affect only one user of Zenmap. Some of them are copied from the `config` subdirectory of the system files when Zenmap is run for the first time. By default, per-user files are in `*`<HOME>`*/.zenmap`[]() on Unix and Mac OS X, where *`<HOME>`* stands for the current user's home directory. They are in `C:\Users\*`<USER>`*\.zenmap` on Windows Vista and Windows 7. On previous versions of Windows they are in `C:\Documents and Settings\*`<USER>`*\.zenmap`. Here *`<USER>`* is the name of the current user. Use the `--confdir` option to use a different directory.

[`recent_scans.txt` ]()

[ This contains a list of file names of recently saved scans. These scans are shown under the “Scan” menu. Scans must have been saved to a file to appear here. See ]()[the section called “Saving and Loading Scan Results”](https://nmap.org/book/zenmap-saving.html). If this file doesn't exist it is created when Zenmap is run.

[`scan_profile.usp` ]()

[ This file contains descriptions of scan profiles, including the defaults and user-created profiles. I recommend using the profile editor (see ]()[the section called “The Profile Editor”](https://nmap.org/book/zenmap-profile-editor.html)) to make changes to this file. This file is copied from the system configuration directory the first time Zenmap is run.

[`target_list.txt` ]()

[ This file contains a list of recently scanned targets. If it doesn't exist it is created when Zenmap is run. ]()

[]()[ `zenmap.conf`]()

[ This is Zenmap's main configuration file. It holds the settings for a particular user's copy of Zenmap and is discussed in more detail in ]()[the section called “Description of `zenmap.conf`”](https://nmap.org/book/zenmap-conf.html).

[`zenmap.db` ]()

[]()[ ]()[ This is the database of recent scans, as described in ]()[the section called “The Recent Scans Database”](https://nmap.org/book/zenmap-saving.html#zenmap-db). It is created if it doesn't already exist.

[`zenmap_version` ]()

[ This file contains the version of Zenmap that was used to create this per-user configuration directory. It may be helpful to compare the version number in this file with the file of the same name in the system configuration directory if you suspect a version conflict. It is simply copied from the system configuration the first time Zenmap is run. ]()

### [Output Files]() ###

[ Whenever a scan is run, Zenmap instructs Nmap to put XML output in a temporary file so that Zenmap can parse it. Normally the XML output file is deleted when the scan is finished. However, if the command line in Zenmap contains an `-oX`]()[]() or `-oA`[]() option, XML output is written to the named file instead, and that file isn't deleted when the scan completes. In other words, `-oX` and `-oA` work the way you would expect. `-oG`[](), `-oN`[](), and `-oS`[]() work too, even though Zenmap doesn't use the output files produced by those options.

 There is one important thing to note in Zenmap's handling of these filenames. Percent characters (%) are escaped to keep them from being interpreted as `strftime`-like[]() format specifiers (see [the section called “Controlling Output Type”](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-flags-type)). This is because Zenmap must know exactly what name Nmap will use for its output file. If in Zenmap you type `-oX scan-%T-%D.xml`, the output file will be saved in the file `scan-%T-%D.xml`, not `scan-144840-121307.xml` or whatever it would have been based on the current time and date if you were executing Nmap directly.

[]()

---

[Prev](https://nmap.org/book/zenmap-lang.html)Zenmap in Your Language

[Up](https://nmap.org/book/zenmap.html)Chapter 12. Zenmap GUI Users' Guide

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/zenmap-conf.html)Description of zenmap.conf