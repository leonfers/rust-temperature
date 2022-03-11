use std::io;

fn main() {
    'outer: loop {
        let operation: fn(u32) -> f64;
        'inner: loop {
            println!(
                "Para converter de Celsius para Fahrenheit digite \"C\", para o inverso \"F\" e para sair \"Q\" !"
            );
            let mut option = String::new();
            io::stdin()
                .read_line(&mut option)
                .expect("É obrigado escolher uma opção!");
            let option = option.trim();
            if option.eq_ignore_ascii_case("c") {
                operation = celsius_to_fahrenheit;
                break 'inner;
            } else if option.eq_ignore_ascii_case("f") {
                operation = fahrenheit_to_celsius;
                break 'inner;
            } else if option.eq_ignore_ascii_case("q") {
                break 'outer;
            } else {
                println!("Opção inválida!({})", option)
            }
        }

        'inner2: loop {
            println!("Digite a temperatura!");
            let mut converted_temperature = String::new();
            io::stdin()
                .read_line(&mut converted_temperature)
                .expect("Um valor é requerido!");
            let converted_temperature: u32 = match converted_temperature.trim().parse() {
                Ok(num) => num,
                Err(num) => {
                    println!("Digite um valor válido de temperatura!({})", num);
                    continue 'inner2;
                }
            };

            println!(
                "A temperatura convertida é {}",
                operation(converted_temperature)
            );
            break 'outer;
        }
    }
    println!("Até mais o/");
}

fn celsius_to_fahrenheit(temperature: u32) -> f64 {
    (temperature as f64) * (9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(temperature: u32) -> f64 {
    ((temperature as f64) - 32.0) / (9.0 / 5.0)
}

#[test]
fn test_celcius_to_fahrenheit() {
    assert_eq!(celsius_to_fahrenheit(0), 32.0);
}

#[test]
fn test_fahrenheit_to_celcius() {
    assert_eq!(fahrenheit_to_celsius(32), 0.0);
}
