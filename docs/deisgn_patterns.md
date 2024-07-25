# Design Patterns

## Tokens

- Structure: `|type<:value>|`
- Types
  - 🟥 Keyword (`k`) `keyword`
  - Literal (`l`) `[type:value]`
    - 🟨 String (str)
    - 🟦 Number (num)
  - 🟩 Symbol (`s`) `['symbol']`
  - 🟪 Semicolon (`;`)
  - 🟪 End of line (`EOL`)
  - 🟪 End of file (`EOF`)

## Lexer

- Lexer Error: `type >> error message`
- Error Types
  - TokenizerError: During the process of transcribing the characters to tokens
    LexerError: Invalid patterns (as missing semicolons)

## Parser

- Keywords
  - 🟥 Built-in
    ⬜ Variables

### Nodes

- Expressions `[ expr ]`
  - Literal
  - Operation
- Statements `{ statement }`

### Data Values (`val`)

- 🟨 String (str)
- 🟦 Number (num)
