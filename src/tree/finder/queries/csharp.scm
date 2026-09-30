[
  "if"
  "else"
  "for"
  "foreach"
  "while"
  "do"
  "switch"
  "case"
  "default"
  "break"
  "continue"
  "return"
  "throw"
  "try"
  "catch"
  "finally"
  "new"
  "this"
  "base"
  "typeof"
  "sizeof"
  "is"
  "as"
  "in"
  "out"
  "ref"
  "class"
  "interface"
  "struct"
  "enum"
  "namespace"
  "using"
] @keyword

(modifier) @keyword

(comment) @comment

(string_literal) @string
(interpolated_string_expression) @string

(method_declaration
  name: (identifier) @function)

(invocation_expression
  function: (identifier) @function)

(predefined_type) @type
