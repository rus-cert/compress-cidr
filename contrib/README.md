## List all IPv6 addresses which are not assigned to a RIR for unicast

First extract all IPv6 unicast address assignments:

    saxonb-xslt http://www.iana.org/assignments/ipv6-unicast-address-assignments/ipv6-unicast-address-assignments.xml ipv6-unicast-assigns.xslt

Then pipe it through `compress-cidr -6 -i -a` to extract all excluded ranges:

    ... | ../target/release/compress-cidr -6 -i -a

The addresses assigned to the various [Regional Internet registries
(RIR)](https://en.wikipedia.org/wiki/Regional_Internet_registry) are not
all actively used; check the individual RIR for details.
