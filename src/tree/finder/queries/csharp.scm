[
  [
  "as"
  "break"
  "case"
  "class"
  "continue"
  "else"
  "finally"
  "for"
  "if"
  "in"
  "is"
  "return"
  "try"
  "while"
] @keyword] @keyword

[
  (raw_string_literal)
] @string

(comment) @comment

(method_declaration
  name: (identifier) @function)
