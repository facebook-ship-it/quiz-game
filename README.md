# quiz-game

A small command-line quiz in Rust. It asks five multiple-choice questions, tells you right away whether each answer was correct and prints your score at the end. No dependencies.

## Run

Requires Rust (edition 2021).

```sh
cargo run
```

Answer each question by typing 1-4 and pressing Enter. You can also pipe answers in:

```sh
printf '2\n3\n1\n4\n2\n' | cargo run -q
```
