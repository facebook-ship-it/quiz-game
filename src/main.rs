use std::io::{self, BufRead, Write};

struct Question {
    prompt: &'static str,
    options: [&'static str; 4],
    answer: usize,
}

const QUESTIONS: [Question; 5] = [
    Question { prompt: "Which planet is closest to the Sun?", options: ["Venus", "Mercury", "Mars", "Earth"], answer: 1 },
    Question { prompt: "How many bits are in a byte?", options: ["4", "16", "8", "32"], answer: 2 },
    Question { prompt: "What is the chemical symbol for gold?", options: ["Au", "Ag", "Gd", "Go"], answer: 0 },
    Question { prompt: "Which ocean is the largest?", options: ["Atlantic", "Indian", "Arctic", "Pacific"], answer: 3 },
    Question { prompt: "What year did the first Moon landing happen?", options: ["1965", "1969", "1972", "1959"], answer: 1 },
];

// Reads lines until one is a number 1..=4; None on end of input.
fn read_choice(lines: &mut impl Iterator<Item = io::Result<String>>) -> Option<usize> {
    loop {
        print!("Your answer (1-4): ");
        io::stdout().flush().ok()?;
        let line = lines.next()?.ok()?;
        match line.trim().parse::<usize>() {
            Ok(n @ 1..=4) => return Some(n - 1),
            _ => println!("Please type a number from 1 to 4."),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut score = 0;

    println!("Quiz time! {} questions.\n", QUESTIONS.len());
    for (i, q) in QUESTIONS.iter().enumerate() {
        println!("Q{}. {}", i + 1, q.prompt);
        for (n, option) in q.options.iter().enumerate() {
            println!("  {}) {}", n + 1, option);
        }
        let Some(choice) = read_choice(&mut lines) else {
            println!("\nNo more input, stopping early.");
            break;
        };
        if choice == q.answer {
            score += 1;
            println!("Correct!\n");
        } else {
            println!("Wrong, it was {}.\n", q.options[q.answer]);
        }
    }

    let percent = score * 100 / QUESTIONS.len();
    println!("Final score: {score}/{} ({percent}%)", QUESTIONS.len());
}
