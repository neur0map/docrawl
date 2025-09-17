---
title: "Acknowledgements | Nmap Network Scanning"
source_url: https://nmap.org/book/acknowledgements.html
fetched_at: 2025-09-17T16:38:12.561209+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Preface](https://nmap.org/book/preface.html)
* Acknowledgements

[Prev](https://nmap.org/book/pref-rfc.html)

[Next](https://nmap.org/book/tcpip-ref.html)

Acknowledgements
----------

When I first floated the idea of writing an Nmap book to the*nmap-hackers* mailing list, I was inundated with
suggestions and offers to help. This outpouring of enthusiasm
convinced me to proceed. My complete naivety about how much work was
involved also contributed to my decision. It has been quite an
undertaking, but what kept me going chapter by chapter was a private
review group called the *nmap-writers*. They
provided invaluable feedback, advice, and detailed review notes
throughout the process. In particular, I would like to thank the
following people:

* **David Fifield** is listed first (everyone else is alphabetical) because he was a tremendous help during the book writing process. He solved a number of technical DocBook problems, created many of the final illustrations from my terrible drafts, dramatically improved the index, helped with proofreading, and even wrote [Chapter 12, *Zenmap GUI Users' Guide*](https://nmap.org/book/zenmap.html).

* **Matt Baxter** allowed the use of his beautiful TCP/IP header diagrams (in [the section called “TCP/IP Reference”](https://nmap.org/book/tcpip-ref.html)). Several other diagrams in this book were done in that style to match.

* **Saurabh Bhasin** contributed detailed feedback on a regular basis.

* **Mark Brewis** could always be counted on for good advice.

* **Ellen Colombo** was a big help from the beginning.

* **Patrick Donnelly** helped improve [Chapter 9, *Nmap Scripting Engine*](https://nmap.org/book/nse.html).

* **Brandon Enright** printed out the whole book and reviewed it chapter by chapter.

* **Brian Hatch** has always been a big help.

* **Loren Heal** was a continual source of ideas.

* **Lee “MadHat” Heath** wrote [the section called “MadHat in Wonderland”](https://nmap.org/book/nmap-overview-and-demos.html#madhat-story) and also an early version of [the section called “Grepable Output (`-oG`)”](https://nmap.org/book/output-formats-grepable-output.html).

* **Dan Henage** provided advice and proofread numerous chapters.

* **Tor Houghton** reviewed every chapter, probably giving me more feedback than anyone else.

* **Doug Hoyte** documented the many Nmap features he added, and also handled most of the book indexing.

* **Marius Huse Jacobsen** reviewed many chapters, providing detailed feedback.

* **Kris Katterjohn** performed thorough reviews of several chapters.

* **Eric Krosnes** sent useful technical review feedback and also regularly nagged me about book progress. This was helpful since I didn't have a traditional editor to do so.

* **Vlad Alexa Mancini** created the Nmap eye logo for the cover (and the Nmap web site).

* **Michael Naef** kindly reviewed many chapters.

* **Bill Pollock** of No Starch Press was always happy to provide advice and answer book publishing questions based on his decades of experience.

* **David Pybus** was one of the most frequent contributors of ideas and proofreading.

* **Tyler Reguly** helped by reviewing multiple chapters just when it was most needed.

* **Chuck Sterling** provided both high level advice and detailed proofreading of several chapters.

* **Anders Thulin** provided detailed reviews of many chapters.

* **Bennett Todd** sent dozens of suggestions.

* **Diman Todorov** wrote an initial draft of [Chapter 9, *Nmap Scripting Engine*](https://nmap.org/book/nse.html).

* **Catherine Tornabene** read many chapters and sent extremely detailed feedback.

### Technology Used to Create This Book ###

As an author of open source tools myself,
I'm a big believer in their power and capability. So I made an effort to use them wherever possible in creating this book. I wasn't about to write it in Microsoft Word and then handle layout with Adobe FrameMaker!

Nmap Network Scanning was written with the [GNU Emacs](http://www.gnu.org/software/emacs/) text editor in the [DocBook XML](http://www.docbook.org/) format.

The free online chapters are created from the XML using Norman Walsh's [XSL Stylesheets](http://wiki.docbook.org/topic/DocBookXslStylesheets) and the [xsltproc XSL processor](http://xmlsoft.org/XSLT/).

The print version also uses Norman's stylesheets and xsltproc,
but output to
the [XSL-FO
format](http://en.wikipedia.org/wiki/XSL_Formatting_Objects). An XSL-FO processor is then used to build a PDF. I
would like to
use [Apache FOP](http://xmlgraphics.apache.org/fop/)for this, but
a [footnote-related
bug](https://issues.apache.org/bugzilla/show_bug.cgi?id=37579) prevents this, so I switched to the [RenderX XEP
Engine](http://www.renderx.com/tools/xep.html). XEP is proprietary, but at least it runs on Linux.
I hope to switch back to FOP after the footnote bug is fixed.

Cover layout was done with [Scribus](http://www.scribus.net/) and (due to printing
company format requirements) Adobe InDesign. Raster graphics for the
cover and internal illustrations were created
with [The
Gimp](http://www.gimp.org/), while [Inkscape](http://www.inkscape.org/) was used for vector
graphics.

[Subversion](http://subversion.tigris.org/) was used for revision control and the free web chapters are serviced by [Apache httpd](http://httpd.apache.org/).

---

[Prev](https://nmap.org/book/pref-rfc.html)Request for Comments

[Up](https://nmap.org/book/preface.html)Preface

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/tcpip-ref.html)TCP/IP Reference