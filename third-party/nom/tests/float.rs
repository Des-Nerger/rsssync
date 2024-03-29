#[macro_use]
extern crate nom;

use nom::{digit, IResult};

use std::str;
use std::str::FromStr;

named!(
	unsigned_float<f32>,
	map_res!(
		map_res!(
			recognize!(alt!(
				delimited!(digit, tag!("."), opt!(complete!(digit))) | delimited!(opt!(digit), tag!("."), digit)
			)),
			str::from_utf8
		),
		FromStr::from_str
	)
);

named!(
	float<f32>,
	map!(pair!(opt!(alt!(tag!("+") | tag!("-"))), unsigned_float), |(sign, value): (Option<&[u8]>, f32)| {
		sign.and_then(|s| if s[0] == ('-' as u8) { Some(-1f32) } else { None }).unwrap_or(1f32) * value
	})
);

#[test]
fn unsigned_float_test() {
	assert_eq!(unsigned_float(&b"123.456"[..]), IResult::Done(&b""[..], 123.456));
	assert_eq!(unsigned_float(&b"0.123"[..]), IResult::Done(&b""[..], 0.123));
	assert_eq!(unsigned_float(&b"123.0"[..]), IResult::Done(&b""[..], 123.0));
	assert_eq!(unsigned_float(&b"123."[..]), IResult::Done(&b""[..], 123.0));
	assert_eq!(unsigned_float(&b".123"[..]), IResult::Done(&b""[..], 0.123));
}

#[test]
fn float_test() {
	assert_eq!(float(&b"123.456"[..]), IResult::Done(&b""[..], 123.456));
	assert_eq!(float(&b"+123.456"[..]), IResult::Done(&b""[..], 123.456));
	assert_eq!(float(&b"-123.456"[..]), IResult::Done(&b""[..], -123.456));
}
