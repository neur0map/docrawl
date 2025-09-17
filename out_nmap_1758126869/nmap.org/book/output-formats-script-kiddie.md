---
title: "$crIpT kIddI3 0uTPut (-oS) | Nmap Network Scanning"
source_url: https://nmap.org/book/output-formats-script-kiddie.html
fetched_at: 2025-09-17T16:47:20.055386+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 13. Nmap Output Formats](https://nmap.org/book/output.html)
* $crIpT kIddI3 0uTPut (-oS)

[Prev](https://nmap.org/book/output-formats-normal-output.html)

[Next](https://nmap.org/book/output-formats-xml-output.html)

$crIpT kIddI3 0uTPut (`-oS`)
----------

[]()[]()

Script kiddie output is like interactive output, except that it
is post-processed to better suit the 'l33t HaXXorZ! They previously
looked down on Nmap due to its consistent capitalization and spelling.
It is best understood by example, as given in [Example 13.8](https://nmap.org/book/output-formats-script-kiddie.html#output-formats-ex-script-kiddie).

Example 13.8. A typical example of $crIpt KiDDi3 0utPut

[]()[]()

```
# nmap -T4 -A -oS - scanme.nmap.org

$tartINg NmAp 5.35dC18 ( http://Nmap.org ) at 2010-07-18 15:36 MDT
NmAp $caN r3p0rT f0R sCAnm3.nMAp.0rg (64.13.134.52)
Host is up (0.044z lat3Ncy).
n0t $h0wn: 993 filter3d pOrtS
PORT      sTATe  $erV|CE v3R$|oN
22/tcp    open   $$h     0pEn$$h 4.3 (pRotocOl 2.0)
| $$H-h0stkEy: 1024 60:ac:4D:51:b1:cD:85:09:12:16:92:76:1d:5D:27:6e (D$4)
|_2048 2c:22:75:60:4b:c3:3b:18:a2:97:2c:96:7e:28:dc:dd (R$4)
25/tcp    clO$ed sMTp
53/Tcp    op3n   domain
70/tcp    cl0s3d G0pher
80/tcP    0Pen   httP    ApAchE httpd 2.2.3 ((C3nt0z))
|_HTml-Titl3: Go aHEad And $canm3!
| Http-m3ThodS: P0t3nTiaLly ri$ky m3tHodS: TRACE
|_seE https://nmap.0rg/nSedOc/$cr|ptS/http-m3th0Ds.html
113/tcp   cl0$3d AUth
31337/Tcp CL0$3d 3l!tE
Dev!Ce tYpE: Gen3rAl Purp0$3
Runn1Ng: L|nUX 2.6.X
Oz d3tA1lz: l|nux 2.6.13 - 2.6.31
[Many lines cut for brevity]
Nmap Done: 1 IP addRe$z (1 hO$t up) $cann3d |n 22.50 sec0nds

```

Some humor-impaired people take this option far too seriously,
and scold me for catering to script kiddies. It is simply a joke*making fun* of the script kiddies—they don't
actually use this mode (I hope).

---

[Prev](https://nmap.org/book/output-formats-normal-output.html)Normal Output (-oN)

[Up](https://nmap.org/book/output.html)Chapter 13. Nmap Output Formats

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/output-formats-xml-output.html)XML Output (-oX)