# dCBOR Pattern Expression Syntax (_patex_)

This syntax is inspired by regular expressions but is specifically designed for dCBOR.

The pattern syntax is designed to be flexible and expressive. Patterns can be composed of *value patterns*, *structure patterns*, and combinators known as *meta-patterns*.

Keywords like `tagged`, `number`, etc., are case-sensitive and use lowercase. Patterns can include specific values, ranges, or regexes to match against the corresponding parts of the dCBOR item.

Arrays use bracket syntax `[...]`.

Spaces may used to separate different parts of the pattern.

Parentheses are used to group patterns or specify ranges. The syntax `(pattern)` is really just the repeat pattern with a repeat that matches the pattern exactly once.

The result of successful parsing is a `Pattern` object, which can be used to match against `CBOR` values from the `dcbor` crate.

White space is ignored between tokens, so you can use it to make patterns more readable. The syntax examples below includ white space both to show where it can be used and to show where it *cannot* be used (i.e., between characters of a token like `*?`)

# Value Patterns

All value patterns match atomic CBOR values.

- Boolean
    - `bool`
        - Matches any boolean value.
    - `true`
        - Matches the boolean value `true`.
    - `false`
        - Matches the boolean value `false`.
- ByteString
    - `bstr`
        - Matches any byte string.
    - `h'hex'`
        - Matches a byte string with the specified hex value. Note that the `h'...'` syntax is used to denote hex strings in CBOR diagnostic notation, so we use it here for familiarity.
    - `h'/regex/'`
        - Matches a byte string that matches the specified binary regex.
- Date
    - `date`
        - Matches any date value.
    - `date'iso-8601'`
        - Matches a date value with the specified ISO 8601 format.
    - `date'iso-8601...iso-8601'`
        - Matches a date value within the specified range.
    - `date'iso-8601...'`
        - Matches a date value greater than or equal to the specified ISO 8601 date.
    - `date'...iso-8601'`
        - Matches a date value less than or equal to the specified ISO 8601 date.
    - `date'/regex/'`
        - Matches a date value that matches the specified regex.
- Known Value
    - `known`
        - Matches any known value. (See the `known-values` crate for more information.)
    - `'value'`
        - Matches the specified known value, which is a u64 value. dCBOR prints known values enclosed in single quotes, so we use that syntax here for familiarity. Note: This is a non-prefixed single-quoted pattern.
    - `'name'`
        - Matches the known value with the specified name. Again we use single quotes here for familiarity. Note: This is a non-prefixed single-quoted pattern.
    - `'/regex/'`
        - Matches a known value with a name that matches the specified regex. We do not use the single quotes here. Note: This is a non-prefixed single-quoted pattern.
- Null
    - `null`
        - Matches the null value.
- Number
    - `number`
        - keyword `number` matches any number.
    - `value`
        - Bare numeric value matches the specified number.
    - `value...value`
        - Matches a number within the specified range.
    - `>=value`
        - Matches a number greater than or equal to the specified value.
    - `<=value`
        - Matches a number less than or equal to the specified value.
    - `>value`
        - Matches a number greater than the specified value.
    - `<value`
        - Matches a number less than the specified value.
    - `NaN`
        - Matches the NaN (Not a Number) value.
    - `Infinity`
        - Matches the Infinity value.
    - `-Infinity`
        - Matches the negative Infinity value.
- Text
    - `text`
        - Matches any text value.
    - `"string"`
        - Matches a text value with the specified string. dCBOR diagnostic notation uses double quotes for text strings, so we use that syntax here for familiarity.
    - `/text-regex/`
        - Matches a text value that matches the specified regex. No double quotes are used here, as the regex is not a string but a pattern to match against the text value.
- Digest
    - `digest`
        - Matches any digest value.
    - `digest'hex'`
        - Matches a digest whose value starts with the specified hex prefix. Up to 32 bytes can be specified, which is the length of the full SHA-256 digest.
    - `digest'ur:digest/value'`
        - Matches the specified `ur:digest` value, parsed using `Digest::from_ur_string()`.
    - `digest'/regex/'`
        - Matches a digest value that matches the specified binary regex.

## Structure Patterns

Structure patterns match parts of dCBOR items.
- Array
    - `[*]`
        - Matches any array.
    - `[{n}]`
        - Matches an array with exactly `n` elements.
    - `[{n,m}]`
        - Matches an array with between `n` and `m` elements, inclusive.
    - `[{n,}]`
        - Matches an array with at least `n` elements.
    - `[pattern]`
        - Matches an array where the elements match the specified pattern. The pattern can be a simple pattern, a sequence of patterns, or patterns with repeat quantifiers.
        - Examples:
            - `[42]` - Array containing exactly one element: the number 42
            - `["a", "b", "c"]` - Array containing exactly ["a", "b", "c"] in sequence
            - `[(*)*, 42, (*)*]` - Array containing 42 anywhere within it
            - `[42, (*)*]` - Array starting with 42, followed by any elements
            - `[(*)*, 42]` - Array ending with 42, preceded by any elements
- Map
    - `{*}`
        - Matches any map.
    - `{{n}}`
        - Matches a map with exactly `n` entries.
    - `{{n,m}}`
        - Matches a map with between `n` and `m` entries, inclusive.
    - `{{n,}}`
        - Matches a map with at least `n` entries.
    - `{pattern: pattern, pattern: pattern, ...}`
        - Matches if the specified patterns match the map's keys and values (order isn't important).
- Tagged
    - `tagged`
        - Matches any CBOR tagged value.
    - `tagged ( value, pattern )`
        - Matches the specified CBOR tagged value with content that matches the given pattern. The tag value is a u64 value, formatted as a bare integer with no delimiters apart from the enclosing parentheses.
    - `tagged ( name, pattern )`
        - Matches the CBOR tagged value with the specified name and content that matches the given pattern. The tag name is formatted as a bare alphanumeric string (including hyphens and underscores) with no delimiters apart from the enclosing parentheses.
    - `tagged ( /regex/, pattern )`
        - Matches a CBOR tagged value with a name that matches the specified regex and content that matches the given pattern.

## Meta Patterns

The following meta patterns are available to combine or modify other patterns.

Precedence: Repeat has the highest precedence, followed by And, Not, Sequence, and then Or. Parentheses can be used to group patterns and change precedence.

- And
    - `pattern & pattern & pattern`…
        - Matches if all specified patterns match.
- Any
    - `*`
        - A bare asterisk matches any value.
- Capture
    - `@name ( pattern )`
        - Matches the specified pattern and captures the match for later use with the given name.
- None
    - `NONE`
        - Never matches.
- Not
    - `! pattern`
        - Matches if the specified pattern does not match.
- Or
    - `pattern | pattern | pattern…`
        - Matches if any of the specified patterns match.
- Repeat
    - Greedy — grabs as many repetitions as possible, then backtracks if the rest of the pattern cannot match.
        - `( pattern )` (exactly once, this is used to group patterns)
        - `( pattern )*` (0 or more)
        - `( pattern )?` (0 or 1)
        - `( pattern )+` (1 or more)
        - `( pattern ){ n , m }` (`n` to `m` repeats, inclusive)
    - Lazy — starts with as few repetitions as possible, adding more only if the rest of the pattern cannot match.
        - `( pattern )*?` (0 or more)
        - `( pattern )??` (0 or 1)
        - `( pattern )+?` (1 or more)
        - `( pattern ){ n , m }?` (`n` to `m` repeats, inclusive)
    - Possessive — grabs as many repetitions as possible and never backtracks; if the rest of the pattern cannot match, the whole match fails.
        - `( pattern )*+` (0 or more)
        - `( pattern )?+` (0 or 1)
        - `( pattern )++` (1 or more)
        - `( pattern ){ n , m }+` (`n` to `m` repeats, inclusive)
- Search
    - `SEARCH ( pattern )`
      - Visits every node in the CBOR tree, matching the specified pattern against each node.
- Sequence
    - `pattern, pattern, pattern`
        - Matches if the specified patterns match in sequence, with no other nodes in between.

## Advanced Composite Patterns

The following patterns show examples of combining structure patterns with meta patterns to create complex matching expressions:

- Nested Structure Patterns
    - `tagged ( value , [ pattern ] )`
        - Matches a tagged value containing an array with the specified pattern. The pattern can be simple patterns, sequences, or patterns with repeat quantifiers.
    - `{pattern: [{{n,}}]}`
        - Matches a map where the specified key pattern maps to an array with at least `n` elements.
    - `[{pattern: pattern}, (pattern)*]`
        - Matches an array starting with a map that contains the specified key-value pattern, followed by any other elements.
