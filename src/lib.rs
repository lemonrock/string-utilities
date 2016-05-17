// This file is part of string-utilities. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/string-utilities/master/COPYRIGHT. No part of string-utilities, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of string-utilities. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/string-utilities/master/COPYRIGHT.

use std::cmp::min;
use std::ffi::CString;
use std::io::Error;
use std::io::ErrorKind;

/// Truncates at the specified length, or at slightly less than it so that a partial UTF-8 sequence is not left behind.
/// If the length is less than `length`, then the string is unchanged unless it already ends in an invalud UTF-8 sequence.
pub fn truncate_utf8_safely<'a>(string: &'a str, maximum_length_to_truncate_to: usize) -> &'a str
{
	let mut lowest_length = min(maximum_length_to_truncate_to, string.len());
	while !string.is_char_boundary(lowest_length)
	{
		lowest_length -= 1;
	}
	&string[0..lowest_length]
}

pub fn to_cstr_best_effort(value: &str) -> (CString, Option<Error>)
{
	match CString::new(value)
	{
		Ok(cstring) =>
		{
			(cstring, None)
		},
		Err(nulError) =>
		{
			let endAt = nulError.nul_position();
			let slice = &value[..endAt];
			(CString::new(slice).unwrap(), Some(Error::new(ErrorKind::InvalidInput, "Embedded nul in message")))
		}
	}
}


pub const DefaultUsAsciiReplacementCharacter: u8 = b'*';

pub fn to_8bit_encoding_string<F>(string: &str, maximum_length: usize, replacement_function: F) -> String
where F: Fn(char) -> u8
{
    let maximum_number_of_characters = min(string.len(), maximum_length);
	let mut result: Vec<u8> = Vec::with_capacity(maximum_number_of_characters);
	unsafe { to_8bit_encoding_unsafe(string, maximum_number_of_characters, replacement_function, &mut result) };
	unsafe { String::from_utf8_unchecked(result) }
}

pub fn to_8bit_encoding_useful<F>(string: &str, maximum_length: usize, replacement_function: F) -> Box<Vec<u8>>
where F: Fn(char) -> u8
{
    let maximum_number_of_characters = min(string.len(), maximum_length);
	let mut result: Vec<u8> = Vec::with_capacity(maximum_number_of_characters);
	unsafe { to_8bit_encoding_unsafe(string, maximum_number_of_characters, replacement_function, &mut result) };
	Box::new(result)
}

/// Suitable for conversions to US-ASCII and US-ASCII printable (ie without control codes, space and DEL)
/// Also suitable for conversions to ISO-8859-1 and such like
pub unsafe fn to_8bit_encoding_unsafe<F>(string: &str, maximum_number_of_characters: usize, replacement_function: F, result: &mut Vec<u8>)
where F: Fn(char) -> u8
{
	let mut characters = string.chars();
	for _ in 0..maximum_number_of_characters
	{
	    let character = characters.next().unwrap();
		result.push(replacement_function(character));
	}
}

/// Suitable for RFC 5424, for example
#[inline]
pub fn to_8bit_encoding_replacement_function_us_ascii_printable(character: char, us_ascii_replacement_character: u8) -> u8
{
	debug_assert!(us_ascii_replacement_character <= 127, "us_ascii_replacement_character must be 7-bit, not '{}'", us_ascii_replacement_character);
	
    match character
	{
		'\x21' ... '\x7E' => character as u8,
		_ => us_ascii_replacement_character,
	}
}

#[inline]
pub fn to_8bit_encoding_replacement_function_us_ascii(character: char, us_ascii_replacement_character: u8) -> u8
{
	debug_assert!(us_ascii_replacement_character <= 127, "us_ascii_replacement_character must be 7-bit, not '{}'", us_ascii_replacement_character);
	
    match character
	{
		'\x21' ... '\x7E' => character as u8,
		_ => us_ascii_replacement_character,
	}
}
