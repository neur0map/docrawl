---
title: "Example Script: finger | Nmap Network Scanning"
source_url: https://nmap.org/book/nse-example-scripts.html
fetched_at: 2025-09-17T16:44:18.371201+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 9. Nmap Scripting Engine](https://nmap.org/book/nse.html)
* Example Script: finger

[Prev](https://nmap.org/book/nse-vscan.html)

[Next](https://nmap.org/book/nse-implementation.html)

Example Script: `finger`
----------

[]()

The `finger` script is a perfect example of a short and simple NSE script.

First the information fields are assigned.
A detailed description of what the script
actually does goes in the `description` field.

```
description = [[
Attempts to get a list of usernames via the finger service.
]]

author = "Eddie Bell"

license = "Same as Nmap--See https://nmap.org/book/man-legal.html"

```

The `categories` field is a table
containing all the categories the script belongs to. These are used for
script selection with the `--script` option:

```
categories = {"default", "discovery", "safe"}

```

Every good script comes with a sample of its output in an NSEDoc comment.

```
---
-- @output
-- PORT   STATE SERVICE
-- 79/tcp open  finger
-- | finger:
-- | Welcome to Linux version 2.6.31.12-0.2-default at linux-pb94.site !
-- |  01:14am  up  18:54,  4 users,  load average: 0.14, 0.08, 0.01
-- |
-- | Login      Name                  Tty      Idle  Login Time   Where
-- | Gutek      Ange Gutek           *:0          -     Wed 06:19 console
-- | Gutek      Ange Gutek            pts/1   18:54     Wed 06:20
-- | Gutek      Ange Gutek           *pts/0       -     Thu 00:41
-- |_Gutek      Ange Gutek           *pts/4       3     Thu 01:06

```

You can use the facilities provided by the nselib ([the section called “NSE Libraries”](https://nmap.org/book/nse-library.html)) with `require`. Here
we want to use common communication functions and shorter port rules:

```
require "comm"
require "shortport"

```

We want to run the script against the finger service. So we
test whether it is using the well-known finger port (`79/tcp`), or
whether the service is named “finger” based on version
detection results or in the port number's listing
in `nmap-services`:

```
portrule = shortport.port_or_service(79, "finger")

```

First, the script uses `nmap.new_try` to
create an exception handler that will quit the script in case of an
error. Next, it passes control to `comm.exchange`,
which handles the network transaction. Here we have asked to wait in the communication exchange until we receive at least 100 lines, wait at least 5 seconds, or until the remote side closes the connection. Any errors are handled by the`try` exception handler. The script returns a string
if the call to `comm.exchange()` was successful.

```
action = function(host, port)
	local try = nmap.new_try()

	return try(comm.exchange(host, port, "\r\n",
        	{lines=100, proto=port.protocol, timeout=5000}))
end

```

[]()

---

[Prev](https://nmap.org/book/nse-vscan.html)Version Detection Using NSE

[Up](https://nmap.org/book/nse.html)Chapter 9. Nmap Scripting Engine

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nse-implementation.html)Implementation Details