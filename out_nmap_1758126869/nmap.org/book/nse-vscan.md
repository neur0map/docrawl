---
title: "Version Detection Using NSE | Nmap Network Scanning"
source_url: https://nmap.org/book/nse-vscan.html
fetched_at: 2025-09-17T16:44:02.118508+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 9. Nmap Scripting Engine](https://nmap.org/book/nse.html)
* Version Detection Using NSE

[Prev](https://nmap.org/book/nse-parallelism.html)

[Next](https://nmap.org/book/nse-example-scripts.html)

Version Detection Using NSE
----------

[]()[]()

 The version detection system built into Nmap was designed to efficiently recognize the vast majority of protocols with a simple probe and pattern matching syntax. Some protocols require more complex communication than version detection can handle. A generalized scripting language as provided by NSE is perfect for these tough cases.

 NSE's `version`[]() category contains scripts that enhance standard version detection. Scripts in this category are run whenever you request version detection with `-sV`; you don't need to use `-sC` to run these. This cuts the other way too: if you use `-sC`, you won't get `version` scripts unless you also use `-sV`.

 One protocol which we were unable to detect with normal version detection is Skype[]() version 2. The protocol was likely designed to frustrate detection out of a fear that telecom-affiliated Internet service providers might consider Skype competition and interfere with the traffic. Yet we did find one way to detect it. If Skype receives an HTTP GET request, it pretends to be a web server and returns a 404 error. But for other requests, it sends back a chunk of random-looking data. Proper identification requires sending two probes and comparing the two responses—an ideal task for NSE. The simple NSE script which accomplishes this is shown in [Example 9.14](https://nmap.org/book/nse-vscan.html#nse-skypev2-script).

Example 9.14. A typical version detection script (Skype version 2 detection)

[]()

```
description = [[
Detects the Skype version 2 service.
]]

---
-- @output
-- PORT   STATE SERVICE VERSION
-- 80/tcp open  skype2  Skype

author = "Brandon Enright"
license = "Same as Nmap--See https://nmap.org/book/man-legal.html"
categories = {"version"}

require "comm"
require "shortport"

portrule = function(host, port)
        return (port.number == 80 or port.number == 443 or
                port.service == nil or port.service == "" or
                port.service == "unknown")
               and port.protocol == "tcp" and port.state == "open"
               and port.service ~= "http" and port.service ~= "ssl/http"
               and not(shortport.port_is_excluded(port.number,port.protocol))
end

action = function(host, port)
        local status, result = comm.exchange(host, port,
                "GET / HTTP/1.0\r\n\r\n", {bytes=26, proto=port.protocol})
        if (not status) then
                return
        end
        if (result ~= "HTTP/1.0 404 Not Found\r\n\r\n") then
                return
        end
        -- So far so good, now see if we get random data for another request
        status, result = comm.exchange(host, port,
                "random data\r\n\r\n", {bytes=15, proto=port.protocol})

        if (not status) then
                return
        end
        if string.match(result, "[^%s!-~].*[^%s!-~].*[^%s!-~]") then
                -- Detected
                port.version.name = "skype2"
                port.version.product = "Skype"
                nmap.set_port_version(host, port)
                return  
        end
        return
end

```

 If the script detects Skype, it augments its `port` table with now-known `name` and `product` fields. It then sends this new information to Nmap by calling `nmap.set_port_version`. Several other version fields are available to be set if they are known, but in this case we only have the name and product. For the full list of version fields, refer to the [`nmap.set_port_version` documentation](https://nmap.org/nsedoc/modules/nmap.html).

 Notice that this script does nothing unless it detects the protocol. A script shouldn't produce output (other than debug output) just to say it didn't learn anything.

---

[Prev](https://nmap.org/book/nse-parallelism.html)Script Parallelism in NSE

[Up](https://nmap.org/book/nse.html)Chapter 9. Nmap Scripting Engine

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nse-example-scripts.html)Example Script: finger