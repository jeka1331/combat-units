use std::{clone, collections::HashMap};
use std::error::Error;
use std::{fs, vec};
use tera::{Context, Tera};

use boevoi::{read_positions_from_toml, read_soldiers_from_toml, render_template_to_file, Position, Soldier};

fn main() -> Result<(), Box<dyn Error>> {
    // // Пример данных
    // let mut people = HashMap::new();
    // people.insert("name".to_string(), "John Doe".to_string());
    // people.insert("age".to_string(), "30".to_string());

    // // Создание вектора для примера
    // let mut people_vec: Vec<HashMap<String, String>> = Vec::new();
    // people_vec.push(people);

    // let mut data = HashMap::new();
    // data.insert("people", people_vec);

    // // Вызов функции для рендеринга и сохранения
    // render_template_to_file(
    //     "templates/**/*", // Путь к шаблонам
    //     "table.html",     // Имя шаблона
    //     "output.html",    // Путь для сохранения HTML
    //     &data,
    // )
    let soldiers = read_soldiers_from_toml("data/soldiers.toml")?;
    for value in &soldiers.soldiers {
        println!("Soldier: {:?}", value);
        
    }

    let positions = read_positions_from_toml("data/positions.toml")?;
    for value in &positions.positions {
        println!("Soldier: {:?}", value);
        
    }

    let mut assigned_positions: Vec<(Position, Vec<Soldier>)> = Vec::new();
    Ok(())
}
