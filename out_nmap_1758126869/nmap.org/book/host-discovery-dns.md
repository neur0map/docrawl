---
title: "DNS Resolution | Nmap Network Scanning"
source_url: https://nmap.org/book/host-discovery-dns.html
fetched_at: 2025-09-17T16:39:14.095808+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 3. Host Discovery (“Ping Scanning”)](https://nmap.org/book/host-discovery.html)
* DNS Resolution

[Prev](https://nmap.org/book/host-discovery-find-ips.html)

[Next](https://nmap.org/book/host-discovery-controls.html)

DNS Resolution
----------

[]()

The key focus of Nmap host discovery is determining which hosts
are up and responsive on the network. That narrows down the field of
targets, since you can't hack a host which doesn't exist. But don't
let discovery end there. You wouldn't date someone just
because they're breathing, and selecting boxes on the network to
penetrate deserves special care
too.[]()A great source of information
(about networked hosts, not potential dates) is DNS, the domain name
system. Even security conscious organizations often assign names
which disclose the function of their systems. It's not uncommon to
see wireless access points named `wap` or`wireless`, firewalls named `fw`,`firewall`, or `fw-1`, and
development web servers with not-yet-published content named`dev`, `staging`,`www-int`, or `beta`. Locations or
department names are also often disclosed, as in the company whose
Chicago office firewall is named `fw.chi`.

By default, Nmap performs
reverse-DNS[]()resolution for every IP
which responds to host discovery probes (i.e. those that are online).
If host discovery is skipped with `-Pn`, resolution is
performed for all IPs. Rather than use the slow standard DNS
resolution libraries, Nmap uses a custom stub resolver which
performs dozens of requests in parallel.

While the defaults generally work well, Nmap offers four options
for controlling DNS resolution. They can substantially affect
scanning speed and the amount of information gathered.

`-n` (No DNS resolution) []()

[Tells Nmap to *never* do reverse DNS resolution on the active IP addresses it finds. Since DNS can be slow even with Nmap's built-in parallel stub resolver, this option reduces scanning times.]()

[`-R` (DNS resolution for all targets) ]()[ ]()

[Tells Nmap to *always* do reverse DNS resolution on the target IP addresses. Normally reverse DNS is only performed against responsive (online) hosts.]()

[`--system-dns` (Use system DNS resolver) ]()[ ]()

[By default, Nmap resolves IP addresses by sending queries directly to the name servers configured on your host and then listening for responses. Many requests (often dozens) are performed in parallel to improve performance. Specify this option to use your system resolver instead (one IP at a time via the `getnameinfo` call). This is slow and rarely useful unless you find a bug in the Nmap parallel resolver (please let us know if you do). The system resolver is always used for IPv6 scans.]()

[`--dns-servers *`<server1>`*[,*`<server2>`*[,...]] ` (Servers to use for reverse DNS queries) ]()[ ]()

[By default, Nmap determines your DNS servers (for rDNS resolution) from your resolv.conf file (Unix) or the Registry (Win32). Alternatively, you may use this option to specify alternate servers. This option is not honored if you are using `--system-dns` or an IPv6 scan. Using multiple DNS servers is often faster, especially if you choose authoritative servers for your target IP space. This option can also improve stealth, as your requests can be bounced off just about any recursive DNS server on the Internet.]()

[This option also comes in handy when scanning private networks. Sometimes only a few name servers provide proper rDNS information, and you may not even know where they are. You can scan the network for port 53 (perhaps with version detection), then try Nmap list scans (`-sL`) specifying each name server one at a time with `--dns-servers` until you find one which works.]()

---

[Prev](https://nmap.org/book/host-discovery-find-ips.html)Finding an Organization's IP Addresses

[Up](https://nmap.org/book/host-discovery.html)Chapter 3. Host Discovery (“Ping Scanning”)

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/host-discovery-controls.html)Host Discovery Controls