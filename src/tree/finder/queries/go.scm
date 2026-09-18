[
  "break"
  "default"
  "func"
  "interface" "select"
  "case"
  "defer"
  "go"
  "map"
  "struct"
  "chan"
  "else"
  "goto"
  "package"
  "switch"
  "const"
  "fallthrough"
  "if"
  "range"
  "type"
  "continue"
  "for"
  "import"
  "return"
  "var" 
 ] @keyword

[
 (interpreted_string_literal)
 (raw_string_literal)
] @string

(comment) @comment

(function_declaration
  name: (identifier) @function)

