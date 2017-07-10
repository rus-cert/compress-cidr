extern crate num_traits;

extern crate cidr;
extern crate bitstring;
extern crate bitstring_trees;

use bitstring_trees::set::RadixSet;

pub mod set_def;
pub mod write_lines;

#[cfg(test)]
mod tests;

use write_lines::WriteLinesIter;

use std::str::FromStr;
use std::fmt;

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
	Any,
	IPv4,
	IPv6,
}

struct Config {
	invert: bool,
	complete: bool,
	aggregate: bool,
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
	let brief = format!("Usage: {} [-i] [-a|-c] [-4|-6]", program);
	print_stderr!("{}", opts.usage(&brief));
}

#[cfg(not(feature = "clap"))]
fn get_config() -> Option<Config> {
	use getopts::Options;
	use std::env;

	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();
	opts.optflag("4", "ipv4", "IPv4 mode");
	opts.optflag("6", "ipv6", "IPv6 mode");
	opts.optflag("c", "complete", "Complete covering list of ranges");
	opts.optflag("a", "aggregate", "Aggregate including ranges");
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
	if matches.opt_present("complete") && matches.opt_present("aggregate") {
		println_stderr!("Error: Can either show aggregated or complete list");
		print_usage(&program, opts);
		return None;
	}
	if matches.opt_present("ipv4") && matches.opt_present("ipv6") {
		println_stderr!("Error: Need at most one of --ipv4/--ipv6.");
		print_usage(&program, opts);
		return None;
	}

	Option::Some(Config{
		invert: matches.opt_present("invert"),
		complete: matches.opt_present("complete"),
		aggregate: matches.opt_present("aggregate"),
		protocol: if matches.opt_present("ipv4") {
			ConfigProtocol::IPv4
		} else if matches.opt_present("ipv6") {
			ConfigProtocol::IPv6
		} else {
			ConfigProtocol::Any
		},
	})
}

#[cfg(feature = "clap")]
#[macro_use]
extern crate clap;

#[cfg(feature = "clap")]
fn get_config() -> Option<Config> {
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
		(@group operation =>
			(@arg complete: -c "Complete covering list of ranges")
			(@arg aggregate: -a "Aggregate including ranges")
		)
		(@arg invert: -i "Invert input list")
	).get_matches();

	Option::Some(Config{
		invert: matches.is_present("invert"),
		complete: matches.is_present("complete"),
		aggregate: matches.is_present("aggregate"),
		protocol: if matches.is_present("ipv4") { ConfigProtocol::IPv4 } else { ConfigProtocol::IPv6 },
	})
}

fn read<C>() -> RadixSet<C>
where
	C: bitstring::BitString+FromStr+fmt::Display+Clone,
	<C as FromStr>::Err: fmt::Debug,
{
	use std::io::{self,BufRead};

	let stdin = io::stdin();
	let locked_stdin = stdin.lock();

	let mut s = RadixSet::<C>::default();
	for line in locked_stdin.lines() {
		let l = line.unwrap();
		if !l.is_empty() && l.as_bytes()[0] != ('#' as u8) {
			s.insert(C::from_str(&l).unwrap());
		}
	}

	s
}

fn show_compress<C>(set: &RadixSet<C>, invert: bool)
where
	C: bitstring::BitString+FromStr+fmt::Display+Clone,
	<C as FromStr>::Err: fmt::Debug,
{
	print!("{}", WriteLinesIter::from(
		set_def::Definition::compress(set, invert)
	));
}

fn show_complete<C: bitstring::BitString>(set: &RadixSet<C>, invert: bool)
where
	C: bitstring::BitString+FromStr+fmt::Display+Clone,
	<C as FromStr>::Err: fmt::Debug,
{
	print!("{}", WriteLinesIter::from(
		set_def::Definition::complete(set, invert)
	))
}

fn show_aggregate<C: bitstring::BitString>(set: &RadixSet<C>, invert: bool)
where
	C: bitstring::BitString+FromStr+fmt::Display+Clone,
	<C as FromStr>::Err: fmt::Debug,
{
	for def in set_def::Definition::complete(set, invert).into_iter() {
		if def.include {
			println!("{}", def.prefix);
		}
	}
}

fn show<C: bitstring::BitString>(set: &RadixSet<C>, config: &Config)
where
	C: bitstring::BitString+FromStr+fmt::Display+Clone,
	<C as FromStr>::Err: fmt::Debug,
{
	if config.complete {
		show_complete(set, config.invert);
	} else if config.aggregate {
		show_aggregate(set, config.invert);
	} else {
		show_compress(set, config.invert);
	}
}

fn main() {
	let config = match get_config() {
		Some(o) => o,
		None => return,
	};

	match config.protocol {
		ConfigProtocol::IPv4 => show(&read::<cidr::Ipv4Cidr>(), &config),
		ConfigProtocol::IPv6 => show(&read::<cidr::Ipv6Cidr>(), &config),
		ConfigProtocol::Any => show(&read::<cidr::AnyIpCidr>(), &config),
	}
}
