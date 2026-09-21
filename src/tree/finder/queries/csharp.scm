(modifier) @keyword

(comment) @comment

(string_literal) @string

(method_declaration
  name: (identifier) @function)

(invocation_expression
  function: (identifier) @function)

(invocation_expression
  function: (member_access_expression
    name: (identifier) @function))
