[
  "as"
  "async"
  "await"
  "break"
  "const"
  "continue"
  "dyn"
  "else"
  "enum"
  "extern"
  "fn"
  "for"
  "if"
  "impl"
  "in"
  "let"
  "loop"
  "match"
  "mod"
  "move"
  "pub"
  "ref"
  "return"
  "static"
  "struct"
  "trait"
  "type"
  "unsafe"
  "use"
  "where"
  "while"
  "yield"
] @keyword

(string_literal) @string

(raw_string_literal) @string

(char_literal) @string

(line_comment) @comment

(block_comment) @comment

(function_item
  name: (identifier) @function)
