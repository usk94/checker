mod checker;
use checker::CheckerType;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use ferris_says::say;
use std::io::{self, stdout, BufWriter, Write};

fn main() {
    let validation_types = vec![
        CheckerType::Number,
        CheckerType::Boolean,
        CheckerType::Japanese,
        CheckerType::PhoneNumber,
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("チェックの種類を選んでください")
        .items(&validation_types)
        .default(0)
        .interact_on_opt(&Term::stdout())
        .unwrap();

    let validation_type = &validation_types[selection.unwrap()];

    print!("対象のデータを入力してください: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let result = validation_type.check(input);

    let stdout = stdout();
    let message = if result {
        format!("チェックした結果、{}は{}でした", input, validation_type)
    } else {
        format!(
            "チェックした結果、{}は{}ではありませんでした",
            input, validation_type
        )
    };
    let width = message.chars().count() + 4;
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
