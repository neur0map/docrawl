---
title: "Description of zenmap.conf | Nmap Network Scanning"
source_url: https://nmap.org/book/zenmap-conf.html
fetched_at: 2025-09-17T16:47:01.361776+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 12. Zenmap GUI Users' Guide](https://nmap.org/book/zenmap.html)
* Description of zenmap.conf

[Prev](https://nmap.org/book/zenmap-files.html)

[Next](https://nmap.org/book/zenmap-command-line.html)

Description of `zenmap.conf`
----------

[]()

`zenmap.conf` is the user-specific configuration file for Zenmap. It is a plain text file located in the per-user configuration directory (see [the section called “Per-user Configuration Files”](https://nmap.org/book/zenmap-files.html#zenmap-user-conf)). The syntax is that recognized by the Python [ConfigParser](http://docs.python.org/library/configparser.html) class, which is similar to that of Windows INI files. Sections are delimited by titles in square brackets. Within sections are lines containing `*`<name>`*=*`<value>`*` or `*`<name>`*: *`<value>`*` pairs. An excerpt from a `zenmap.conf` is shown.

```
[output_highlight]
enable_highlight = True

[paths]
nmap_command_path = nmap
ndiff_command_path = ndiff

[search]
search_db = 1
file_extension = xml
store_results = 1
directory =
save_time = 60;days

```

 Some of these settings can be controlled from within Zenmap without editing the configuration file directly.

### Sections of `zenmap.conf` ###

 Boolean values are normalized from `True`, `true`, or `1` to true or anything else to false.

`[paths]`

 The `[paths]` section defines important paths used by Zenmap.

[`nmap_command_path`]()

[The path to the Nmap executable. Whatever the first word is in a command line executed by Zenmap will be replaced by the value of this variable. Its default value of `nmap` is appropriate for most systems. See ]()[the section called “The `nmap` Executable”](https://nmap.org/book/zenmap-files.html#zenmap-executable) for examples.

[`ndiff_command_path`]()

[The path to the Ndiff scan comparison utility. Zenmap uses Ndiff to do scan comparisons; see ]()[the section called “Comparing Results”](https://nmap.org/book/zenmap-compare.html).

`[search]`

 The `[search]` section defines how the search tool (see [the section called “Searching Saved Results”](https://nmap.org/book/zenmap-search.html)) behaves. The names in this section correspond to the options in the “Search options” tab of the search dialog. It has the following names defined.

`directory`

The directory to search for saved scan results files.

`file_extension`

A semicolon-separated list of file name extensions to search.

`search_db`

A Boolean controlling whether to search the recent scans database.

`store_results`[]()

[A Boolean controlling whether to store scan results in the recent scans database. See ]()[the section called “The Recent Scans Database”](https://nmap.org/book/zenmap-saving.html#zenmap-db).

`save_time`

How long to keep scan results in the recent scans database. Results older than this are deleted when Zenmap is closed. The format is a number and a time interval separated by semicolons, for example `60;days` or `1;years`.

`[diff]`

 The `[diff]` section defines how the comparison tool (see [the section called “Comparing Results”](https://nmap.org/book/zenmap-compare.html)) behaves. It has the following names defined.

`diff_mode`

Controls whether comparisons are shown by default in graphical or text mode. Must be either `compare` for graphical mode or `text`.

`colored_diff`

A Boolean that controls if comparisons use color.

`[diff_colors]`

 The `[diff_colors]` section defines the colors used by the comparison tool. It has the following names defined: `unchanged`, `added`, `not_present`, and `modified`, the meanings of which are defined in [the section called “Comparing Results”](https://nmap.org/book/zenmap-compare.html). The value of each of these is a list of three integers in the range 0–65535 representing red, green, and blue in the format `[*`<red>`*, *`<green>`*, *`<blue>`*]`. For example, `[65535, 0, 0]` specifies red.

`[output_highlight]`

 The `[output_highlight]` section contains a single Boolean variable `enable_highlight`, which enables output highlighting when `True` and disables it if `False`.

`[date_highlight]`, `[hostname_highlight]`, `[ip_highlight]`, `[port_list_highlight]`, `[open_port_highlight]`, `[closed_port_highlight]`, `[filtered_port_highlight]`, `[details_highlight]`

 These sections all define the nature of Nmap output highlighting, which is discussed in [the section called “The “Nmap Output” tab”](https://nmap.org/book/zenmap-results.html#zenmap-tab-nmap-output). These are best edited from within Zenmap. Within each of these sections, the following names are defined.

`regex`[]()

[The regular expression that matches the relevant part of the output.]()

[`bold`]()

[A Boolean controlling whether to make this highlight bold.]()

[`italic`]()

[A Boolean controlling whether to make this highlight italic.]()

[`underline`]()

[A Boolean controlling whether to underline this highlight.]()

[`text`]()

[The color of the text in this highlight. The syntax is a list of three integers in the range 0–65535 representing red, green, and blue in the format `[*`<red>`*, *`<green>`*, *`<blue>`*]`. For example, `[65535, 0, 0]` for a red highlight.]()

[`highlight`]()

[The color of the background in this highlight. The syntax is the same as for `text`.]()

[]()

---

[Prev](https://nmap.org/book/zenmap-files.html)Files Used by Zenmap

[Up](https://nmap.org/book/zenmap.html)Chapter 12. Zenmap GUI Users' Guide

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/zenmap-command-line.html)Command-line Options