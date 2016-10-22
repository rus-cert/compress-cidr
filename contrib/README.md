## List all IPv6 addresses which are not assigned for unicast

First extract all IPv6 unicast address assignments:

    saxonb-xslt http://www.iana.org/assignments/ipv6-unicast-address-assignments/ipv6-unicast-address-assignments.xml ipv6-unicast-assigns.xslt

Then pipe it through `compress-cidr -6 -c` and extract all lines with `exclude`:

    ... | ../target/release/compress-cidr -6 -c | grep exclude | cut -d' ' -f2

