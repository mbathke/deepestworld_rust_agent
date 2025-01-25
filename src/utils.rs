const VALUE_KEYWORD_LEN: usize = 7;

/// Parses an html document to find a csrf token of a form.
/// It returns Some(token) or None if it can't find any.
pub fn parse_csrf_token(body: &str) -> Option<&str> {
    // find the index of the string _csrf_token
    let start_index = body.find("_csrf_token"); 

    match start_index {
        Some(start_csrf) => {
            // in case we found the token name, try to find its value index
            let start_value = body[start_csrf..].find("value");
            
            match start_value {
                Some(value_idx) => {
                    let value_idx = start_csrf + value_idx + VALUE_KEYWORD_LEN;
                    // in case we also found the value index, we need to find the closing quote
                    // we assume that 'value="' takes 7 characters so we begin our search after the
                    // opening quote
                    let end_value = body[value_idx..].find('"');

                    match end_value {
                        Some(end_idx) => {
                            let end_idx = value_idx + end_idx;
                            let output = &body[value_idx..end_idx];
                            // Now that we found start and end index of the value, return the slice 
                            return Some(output)
                        },
                        None => None 
                    }
                },
                None => None
            }
        },
        None => None
    }
}

/// Parses a string slice for a keyword and returns a new slice when a semicolon
/// is found like its typically formatted in a Cookie.
///
/// @deprecated
pub fn _parse_cookie<'a>(keyword: &str, string_slice: &'a str) -> Option<&'a str> {
    let start_index = string_slice.find(keyword);

    match start_index {
        Some(start) => {
            let end_index = string_slice[start..].find(';');
            match end_index {
                Some(end) => Some(&string_slice[start..end]),
                None => None,
            }
        }
        None => return None,
    }
}
