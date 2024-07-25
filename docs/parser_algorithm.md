# Parsing Algorithm

## Nomenclature

- Node
  - Can be represented as a list of tokens: `node[token_pos]`
  - `prev`: The node before `current`
  - `current`: The current node to parse
  - `next`: The next node to parse, after `current`

## Main loop (parse_tree -> AST)

Iteration starts at `current[0]`, ends at `next[0]`.

1. Peek & match (`:t = current[0]`)
    1. keyword
        - 'let': SVariableDeclaration
    2. EOL: `NEXT_TOKEN + CONTINUE`
    3. EOF: `STOP`
2. Try to parse a expression, otherwise: `EXCEPTION`

```mermaid
---
title: Parsing a node from tokens
---

flowchart LR

what_token[What token is it?] -- keyword --> what_keyword[What keyword is it?]
what_token -- EOL --> NEXT --> what_token
what_token -- EOF --> STOP
what_token -- (default) --> try_parse_expression[Try to parse \n an expression]

what_keyword -- let --> node_var_ref[VariableReferenceNode]
what_keyword -- (default) --> EXCEPTION

try_parse_expression -- Ok --> node_expression[ExpressionNode]
try_parse_expression -- Err --> EXCEPTION

```

## Parse Expression

```mermaid
---
title: Parsing an expression from tokens
---

flowchart TD

what_token{What \n token \n is it?}

what_token -- keyword --> node_var_ref[VariableReferenceNode]
what_token -- literal --> node_lit[LiteralNode]
what_token -- (default) --> EXCEPTION

node_var_ref --> next_token_if_op{Is the next \n token an \n operator?}
node_lit --> next_token_if_op

next_token_if_op -- YES --> node_op
next_token_if_op -- NO --> data_value[Data Value]

node_op[OperationNode]

node_op -- With the next token --> what_token

```
