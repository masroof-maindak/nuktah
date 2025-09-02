## Current Algorithm

Pick up a lexeme, starting from a globally incrementing index, until a delimiter is encountered. If you're standing on the delimiter to begin with, you're done. Else, return a substring till the delimiter's index.

Classify the token.

Check the last prior token's type, and if a 'promotion'/'merging' is needed e.g `<` followed up by another `<` becomes `<<`, or if the last token is a string literal ending with a backslash (`\\`) and the new token is a T\_QUOTES, then pop the previous token, merge it with the new token that is currently being considered, and push it to the vector of tokens.

## Objectively Better Algorithm

Scan character by character, and build up a token as you go. This is:

1. more powerful as it allows for more robust error-handling e.g in the case of float edge cases (`12.a`, `12a5`, `12.`, etc.)
2. Easier to modify by virtue of lookaheads e.g
    - The `comment_started` bool can simply be ignored if we encounter a `\"` inside a string literal -- or any other escape sequence for that matter; the only difference would be replacing `\"` with `\\\"`, as opposed to turning `\n` to `\\n` 
    - When a binary operation's delimiter is encountered, check the next one as it might just be a boolean operation i.e `&` to `&&` for instance

---

I probably won't shift though, because min-maxing my lexer further is pointless, and I have some peak (as per my humble opinion) iterator usage on display.

This lexer implementation also has a soft-spot in my heart by virtue of teaching me about iterators, so it gets to stay as is. I'm largely documenting it in case any lost soul stumbles upon it by accident.