---
title: "Output to a Database | Nmap Network Scanning"
source_url: https://nmap.org/book/output-formats-output-to-database.html
fetched_at: 2025-09-17T16:47:37.556386+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 13. Nmap Output Formats](https://nmap.org/book/output.html)
* Output to a Database

[Prev](https://nmap.org/book/output-formats-cpe.html)

[Next](https://nmap.org/book/output-formats-output-to-html.html)

Output to a Database
----------

[]()

A common desire is to output Nmap results to a database for
easier queries and tracking. This allows users from an individual
penetration tester[]()to an international enterprise to store all of their scan
results and easily compare them. The enterprise might run large scans
daily and schedule queries to mail administrators of newly open ports or
available machines. The penetration tester might learn of a new vulnerability
and search all of his old scan results for the affected application so
that he can warn the relevant clients. Researchers may scan millions
of IP addresses and keep the results in a database for easy real-time
queries.

While these goals are laudable, Nmap offers no direct database
output functionality. Not only are there too many different database types
for me to support them all, but users' needs vary so dramatically that
no single database schema is suitable. The needs of the enterprise,
pen-tester, and researcher all call for different table
structures.

For projects large enough to require a database, I recommend
deciding on an optimal DB schema first, then writing a simple program
or script to import Nmap XML data appropriately. Such scripts often
take only minutes, thanks to the wide availability of XML parsers and
database access modules. Perl often makes a good choice, as it offers
a powerful database abstraction layer and also custom Nmap XML
support. [the section called “Manipulating XML Output with Perl”](https://nmap.org/book/output-formats-xml-with-perl.html) shows how
easily Perl scripts can make use of Nmap XML data.

Another option is to use a custom Nmap database support patch.
One example is [nmap-sql](http://sourceforge.net/projects/nmapsql), which
adds MySQL[]()logging functionality into Nmap itself. The downsides are
that it currently only supports the MySQL database and it must be frequently ported
to new Nmap versions. An XML-based approach, on the other hand, is
less likely to break when new Nmap versions are released.

Another option
is [PBNJ](http://pbnj.sourceforge.net/), a suite of
tools for monitoring changes to a network over time. It stores scan
data such as online hosts and open ports to a database (SQLite, MySQL
or Postgres). It offers a flexible querying and alerting system for
accessing that data or displaying changes.

---

[Prev](https://nmap.org/book/output-formats-cpe.html)Common Platform Enumeration (CPE)

[Up](https://nmap.org/book/output.html)Chapter 13. Nmap Output Formats

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/output-formats-output-to-html.html)Creating HTML Reports