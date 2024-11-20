mod checker;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use ferris_says::say;
use std::io::{self, stdout, BufWriter, Write};

fn main() {
    let validation_types = vec!["数値", "真偽値", "日本語", "電話番号"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("チェックの種類を選んでください")
        .items(&validation_types)
        .default(0)
        .interact_on_opt(&Term::stdout())
        .unwrap();

    let validation_type = validation_types[selection.unwrap()];

    print!("対象の文字列を入力してください: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let result = match validation_type {
        "数値" => checker::check_number(input),
        "真偽値" => checker::check_boolean(input),
        "日本語" => checker::check_japanese(input),
        "電話番号" => checker::check_phone_number(input),
        _ => unreachable!(),
    };

    let stdout = stdout();
    let message = if result {
        format!("チェックした結果、これはちゃんと{}でした", validation_type)
    } else {
        format!(
            "チェックした結果、これは{}ではありませんでした",
            validation_type
        )
    };
    let width = message.chars().count() + 4;
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
