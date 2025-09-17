---
title: "Saving and Loading Scan Results | Nmap Network Scanning"
source_url: https://nmap.org/book/zenmap-saving.html
fetched_at: 2025-09-17T16:45:32.557399+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 12. Zenmap GUI Users' Guide](https://nmap.org/book/zenmap.html)
* Saving and Loading Scan Results

[Prev](https://nmap.org/book/zenmap-results.html)

[Next](https://nmap.org/book/zenmap-topology.html)

Saving and Loading Scan Results
----------

[]()[]()

 To save an individual scan to a file, choose “Save Scan” from the “Scan” menu (or use the keyboard shortcut **ctrl**+**S**). If there is more than one scan into the inventory you will be asked which one you want to save. On the save dialog, you have the choice of saving in “Nmap XML format” (`.xml` extension) or “Nmap text format” (`.nmap` extension). The XML format is the only format that can be opened again by Zenmap; if you save in text format you will not be able to open the file again. Nmap output formats are covered in [the section called “XML Output (`-oX`)”](https://nmap.org/book/output-formats-xml-output.html).

 You can save every scan in an inventory with “Save All Scans to Directory” under the “Scan” menu (**ctrl**+**alt**+**S**). When saving an inventory for the first time, you will commonly create a new directory using the “Create Folder” button in the save dialog. In subsequent saves you can continue saving to the same directory. To reduce the chance of overwriting unrelated scan files, the save-to-directory function will refuse to continue if the chosen directory contains a file that doesn't belong to the inventory. If you are sure you want to save to that directory, delete any offending files and then save again.

[]()[]()

 Saved results are loaded by choosing “Open Scan” from the “Scan” menu, or by typing the **ctrl**+**O** keyboard shortcut. In the file selector, the “Open” button opens a single scan, while the “Open Directory” button opens every file in the chosen directory (perhaps created using “Save All Scans to Directory”).

“Open Scan” opens loaded scans in a new window, thereby creating a new inventory. To merge loaded scans into the current inventory instead, use “Open Scan in This Window”.

### The Recent Scans Database ###

[]()[]()

 Scan results that are not saved to a file are automatically stored in a database. Scan results that are loaded from a file, and are then modified (such as by the addition of a host comment) but not re-saved, are also stored in the database. The database is stored in a file called `zenmap.db` and its location is platform-dependent (see [the section called “Files Used by Zenmap”](https://nmap.org/book/zenmap-files.html)). By default, scans are kept in the database for 60 days and then removed. This time interval can be changed by modifying the value of the `save_time` variable in the `[search]` section of `zenmap.conf` (see [the section called “Description of `zenmap.conf`”](https://nmap.org/book/zenmap-conf.html)).

 Zenmap's search interface, because it searches the contents of the recent scans database by default, doubles as a database viewer. On opening the search window every scan in the database is shown. The list of scans may then be filtered by a search string. See [the section called “Searching Saved Results”](https://nmap.org/book/zenmap-search.html).

---

[Prev](https://nmap.org/book/zenmap-results.html)Interpreting Scan Results

[Up](https://nmap.org/book/zenmap.html)Chapter 12. Zenmap GUI Users' Guide

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/zenmap-topology.html)Surfing the Network Topology