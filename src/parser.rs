use regex::Regex;

use crate::error::Error;
use crate::Result;

/// Parses input html to find the end of a JavaScript object.
/// 
///     :param str html:
///         HTML to be parsed for an object.
///     :param str preceding_regex:
///         Regex to find the string preceding the object.
///     :rtype dict:
///     :returns:
///         A dict created from parsing the object.
pub(crate) fn parse_for_object<'a>(html: &'a str, regex: &Regex) -> Result<&'a str> {
    let json_obj_start = regex
        .find(html)?
        .end();

    Ok(json_object(html.get(json_obj_start..)?)?)
}

#[inline]
fn json_object(mut html: &str) -> Result<&str> {
    html = html.trim_start_matches(|c| c != '{');
    if html.is_empty() { return Err(Error::UnexpectedResponse); }

    let mut stack = vec![b'{'];
    let mut skip = false;

    let (i, _c) = html
        .as_bytes()
        .iter()
        .enumerate()
        .skip(1)
        .find(
            |(_i, &curr_char)| find_json_object(curr_char, &mut skip, &mut stack)
        )?;

    let full_obj = html
        .get(..=i)
        .expect("i must always mark the position of a '}' char");

    Ok(full_obj)
}

#[inline]
fn find_json_object(curr_char: u8, skip: &mut bool, stack: &mut Vec<u8>) -> bool {
    if *skip {
        *skip = false;
        return false;
    }

    let context = *stack
        .last()
        .expect("stack must start with len == 1, and find mut end, when len == 0");

    match curr_char {
        b'}' if context == b'{' => { stack.pop(); }
        b']' if context == b'[' => { stack.pop(); }
        b'"' if context == b'"' => { stack.pop(); }

        b'\\' if context == b'"' => { *skip = true; }

        b'{' if context != b'"' => stack.push(b'{'),
        b'[' if context != b'"' => stack.push(b'['),
        b'"' if context != b'"' => stack.push(b'"'),

        _ => {}
    }

    stack.is_empty()
}
