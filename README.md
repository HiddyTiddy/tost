# tost
tost is pronounced like toast but spelt like tost; fuck you

# TODO
which language
- C
- python
- rust
    - needs oxygne, metla (irno), wo'ooh
    - https://doc.rust-lang.org/stable/book/
- brainfuck
- matlab
- scratch
- tost :)


https://switzerb.github.io/imposter/algorithms/2021/01/12/dijkstra-two-stack.html

## 1. Lexer
started
TODO: 
- better conversion from pure text to tokens
    - -> numbers 
      - [✅] int (123, 0x1a4, 0b1010011010, 0o077) => actually just store as int and not as string
      - [✅] floats (0.123, 1.2e-1, 0., .0) => store as f64
      - that's it!
    - -> string literals
      - "string important spaces and also important \\"escaped\\" quotes"
    - -> чaracters
      - 'ч'
    - -> booleans
      - good / bad
      - true / false
      - yay / nay
      - yes / no
    - -> :{, }:, {:, :} (maybe)
    - -> if, while, whatever // actually, let's add a 'whatever' keyword
    - -> types? but only basic types? but actually nvm not yet
    - -> Toast
## 2. Parser
started
TODO:
- a lot
- functions with args
- calling functions
- check if :{ matches with }: and {: with :}
- types? or is that in [3.]
- if
- while
- (for) let's first just live with while, bc i said so.
- arithmetic
## 3. AST Generator
## 4. Assembly Generator
TODO:
im actually quite happy already if we have a program that just does x = 5; and sucessfully generates the assembly for that

## QUALITY OF LIFE
- syntax highlighting (idk yet how)
  - vim
  - vscode