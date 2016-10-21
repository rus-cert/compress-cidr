extern crate num_traits;

pub mod bitstrings;
pub mod cidr;
pub mod radixset;

use std::net::{Ipv4Addr,Ipv6Addr};
use cidr::IpCidr;
use std::str::FromStr;

macro_rules! print_stderr(
	($($arg:tt)*) => { {
		use std::io::Write;
		let r = write!(&mut ::std::io::stderr(), $($arg)*);
		r.expect("failed printing to stderr");
	} }
);

macro_rules! println_stderr(
	($($arg:tt)*) => { {
		use std::io::Write;
		let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
		r.expect("failed printing to stderr");
	} }
);

enum ConfigProtocol {
	IPv4,
	IPv6,
}

struct Config {
	invert: bool,
	complete: bool,
	protocol: ConfigProtocol,
}
use std::option::Option;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESC: &'static str = "Converts (positive) CIDR list into minimal list of positive and negative definitions";

#[cfg(not(feature = "clap"))]
extern crate getopts;

#[cfg(not(feature = "clap"))]
fn print_usage(program: &str, opts: getopts::Options) {
	print_stderr!("{} {}\n{}\n{}\n\n", NAME, VERSION, AUTHORS, DESC);
	let brief = format!("Usage: {} [-i] <-4|-6>", program);
	print_stderr!("{}", opts.usage(&brief));
}

#[cfg(not(feature = "clap"))]
fn get_options() -> Option<Config> {
	use getopts::Options;
	use std::env;

	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();
	opts.optflag("4", "ipv4", "IPv4 mode");
	opts.optflag("6", "ipv6", "IPv6 mode");
	opts.optflag("c", "complete", "Complete covering list of ranges");
	opts.optflag("i", "invert", "Invert input list");
	opts.optflag("h", "help", "print this help menu");
	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m }
		Err(f) => {
			println_stderr!("{}", f);
			print_usage(&program, opts);
			return None;
		}
	};
	if matches.opt_present("h") || !matches.free.is_empty() {
		print_usage(&program, opts);
		return None;
	}
	if matches.opt_present("ipv4") == matches.opt_present("ipv6") {
		println_stderr!("Need exactly one of --ipv4/--ipv6.");
		print_usage(&program, opts);
		return None;
	}

	Option::Some(Config{
		complete: matches.opt_present("complete"),
		invert: matches.opt_present("invert"),
		protocol: if matches.opt_present("ipv4") { ConfigProtocol::IPv4 } else { ConfigProtocol::IPv6 },
	})
}

#[cfg(feature = "clap")]
#[macro_use]
extern crate clap;

#[cfg(feature = "clap")]
fn get_options() -> Option<Config> {
	let matches = clap_app!(
		@app (clap::App::new(NAME))
		(version: VERSION)
		(author: AUTHORS)
		(about: DESC)
		(@group protocol =>
			(@attributes +required)
			(@arg ipv4: short("4") "IPv4 mode")
			(@arg ipv6: short("6") "IPv6 mode")
		)
		(@arg complete: -c "Complete covering list of ranges")
		(@arg invert: -i "Invert input list")
	).get_matches();

	Option::Some(Config{
		complete: matches.is_present("complete"),
		invert: matches.is_present("invert"),
		protocol: if matches.is_present("ipv4") { ConfigProtocol::IPv4 } else { ConfigProtocol::IPv6 },
	})
}

fn read<A: cidr::IpAddress>() -> radixset::RadixSet<IpCidr<A>> {
	use std::io::{self,BufRead};

	let stdin = io::stdin();
	let locked_stdin = stdin.lock();

	let mut s = radixset::RadixSet::<IpCidr<A>>::default();
	for line in locked_stdin.lines() {
		let l = line.unwrap();
		if !l.is_empty() && l.as_bytes()[0] != ('#' as u8) {
			s.insert(&IpCidr::<A>::from_str(&l).unwrap());
		}
	}

	s
}

fn show_compress<A: cidr::IpAddress>(set: &radixset::RadixSet<IpCidr<A>>, invert: bool) {
	for def in radixset::def::Definition::compress(set, invert).into_iter() {
		println!("{}", &def);
	}
}

fn show_complete<A: cidr::IpAddress>(set: &radixset::RadixSet<IpCidr<A>>, invert: bool) {
	for def in radixset::def::Definition::complete(set, invert).into_iter() {
		println!("{}", &def);
	}
}

fn main() {
	let options = match get_options() {
		Some(o) => o,
		None => return,
	};

	if options.complete {
		match options.protocol {
			ConfigProtocol::IPv4 => show_complete(&read::<Ipv4Addr>(), options.invert),
			ConfigProtocol::IPv6 => show_complete(&read::<Ipv6Addr>(), options.invert),
		}
	} else {
		match options.protocol {
			ConfigProtocol::IPv4 => show_compress(&read::<Ipv4Addr>(), options.invert),
			ConfigProtocol::IPv6 => show_compress(&read::<Ipv6Addr>(), options.invert),
		}
	}
}
