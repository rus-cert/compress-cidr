# Description

Given a set of IP addresses through a list of CIDR ranges, the list of
CIDR ranges can grow very big (especially for IPv6), if small
(sub)ranges are excluded from the set.

`compress-cidr` can generate a list of CIDR rules, included and
excluding ranges from the set; to check whether a single address is
included one needs to take a look at the rule with the longest matching
CIDR rule.

`compress-cidr` can also build the complement of a set, or generate a
list of non-overlapping but covering CIDR rules.

# Build

`compress-cidr` is implemented in [rust](https://www.rust-lang.org) and
uses [cargo](https://crates.io/) for dependency management.

Run `cargo build --release` to build it.

# Examples

## Compress with small sub range excluded

```
# ./target/release/compress-cidr -6 <<EOF
::/3
4000::/2
8000::/1
EOF
```

generates

```
include ::/0
exclude 2000::/3
```

## Generate non-overlapping coverage

```
# ./target/release/compress-cidr -6 -c <<EOF
::/3
4000::/2
8000::/1
EOF
```

generates

```
include ::/3
exclude 2000::/3
include 4000::/2
include 8000::/1
```

## Aggregate list

By creating a non-overlapping coverage and only looking at the include
rules one can get an aggregated list:

```
# ./target/release/compress-cidr -6 -a <<EOF
::/8
100::/8
200::/7
400::/6
800::/5
1000::/4
4000::/2
8000::/1
EOF
```

produces

```
::/3
4000::/2
8000::/1
```
