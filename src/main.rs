#![windows_subsystem = "windows"]
#![warn(clippy::pedantic, elided_lifetimes_in_paths, explicit_outlives_requirements)]
#![allow(
	confusable_idents,
	mixed_script_confusables,
	non_camel_case_types,
	non_snake_case,
	uncommon_codepoints
)]

use {
	combine::{
		char::{char, digit},
		combinator::{eof, many1, optional, parser as p},
		primitives::{ParseResult, Stream},
		Parser,
	},
	std::{
		env, fs,
		io::{self, Write},
		path::PathBuf,
		str::FromStr,
	},
	subparse::{
		get_subtitle_format_by_extension_err, parse_str,
		timetypes::{TimeDelta, TimePoint},
	},
};

fn main() {
	let (filePath, mut argPairs);
	{
		let mut args = env::args().skip(1);
		filePath = PathBuf::from(args.next().unwrap());
		argPairs = Vec::with_capacity(args.len() / 2 + 1);
		(|| -> Option<()> {
			loop {
				let timePoint = args.next()?;
				argPairs.push((
					(p(number_i64), char(':'), p(number_i64), char('.'), p(number_i64), eof())
						.map(|t| TimePoint::from_components(default(), t.0, t.2, t.4))
						.parse(&timePoint[..])
						.unwrap()
						.0,
					usize::from_str(&args.next()?).unwrap() - 1,
				));
			}
		})();
		argPairs.push((TimePoint::from_msecs(default()), usize::MAX));
	};
	let (filePath, mut argPairs) = (filePath.as_path(), argPairs.as_slice());
	let fileContent = &fs::read_to_string(filePath).unwrap();
	let subtFormat = get_subtitle_format_by_extension_err(filePath.extension()).unwrap();
	let subtFile = &mut parse_str(subtFormat, fileContent, default()).unwrap();
	let subtEntries = &mut subtFile.get_subtitle_entries().unwrap();
	{
		let mut timeDelta = TimeDelta::from_msecs(0);
		for (i, subtEntry) in subtEntries.iter_mut().enumerate() {
			if i == argPairs[0].1 {
				timeDelta = argPairs[0].0 - subtEntry.timespan.start;
				argPairs = &argPairs[1..];
			}
			subtEntry.timespan += timeDelta;
		}
	}
	subtFile.update_subtitle_entries(subtEntries).unwrap();
	io::stdout().write_all(&subtFile.to_data().unwrap()).unwrap();
}

fn default<T: Default>() -> T {
	Default::default()
}

/// Matches a positive or negative integer number.
fn number_i64<I>(input: I) -> ParseResult<i64, I>
where
	I: Stream<Item = char>,
{
	(optional(char('-')), many1(digit()))
		.map(|(a, c): (Option<_>, String)| {
			// we provide a string that only contains digits: this unwrap should never fail
			let i = i64::from_str(&c).unwrap();
			match a {
				Some(_) => -i,
				None => i,
			}
		})
		.expected("positive or negative number")
		.parse_stream(input)
}
