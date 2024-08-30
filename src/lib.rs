use serde::Deserialize;
use tera::{Tera, Context};
use std::collections::HashMap;
use std::error::Error;
use std::fs;


#[derive(Debug, Deserialize)]
pub struct Soldier {
    rank: String,
    fio: String,
    vzvod: String,
}

#[derive(Debug, Deserialize)]
pub struct SoldiersList {
    pub soldiers: Vec<Soldier>,
}


#[derive(Debug, Deserialize)]
pub struct Position {
    name: String,
    ammo: String,
    member_count: isize,
    vzvod_priority: String,
}

#[derive(Debug, Deserialize)]
pub struct PositionsList {
    pub positions: Vec<Position>,
}

// Функция для чтения и десериализации TOML данных
pub fn read_soldiers_from_toml(file_path: &str) -> Result<SoldiersList, Box<dyn Error>> {
    // Загрузка содержимого файла TOML в строку
    let toml_str = fs::read_to_string(file_path)?;
    
    // Десериализация строки TOML в структуру Rust
    let result: SoldiersList = toml::de::from_str(&toml_str)?;
    
    // Возвращаем результат
    Ok(result)
}

// Функция для чтения и десериализации TOML данных
pub fn read_positions_from_toml(file_path: &str) -> Result<PositionsList, Box<dyn Error>> {
    // Загрузка содержимого файла TOML в строку
    let toml_str = fs::read_to_string(file_path)?;
    
    // Десериализация строки TOML в структуру Rust
    let result: PositionsList = toml::de::from_str(&toml_str)?;
    
    // Возвращаем результат
    Ok(result)
}

/// Функция для рендеринга шаблона и сохранения HTML в файл
pub fn render_template_to_file(
    template_path: &str,
    template_name: &str,
    output_path: &str,
    data: &HashMap<&str, Vec<HashMap<String, String>>>
) -> Result<(), Box<dyn Error>> {
    // Инициализация шаблонизатора Tera и загрузка шаблонов
    let tera = Tera::new(template_path)?;

    // Создание контекста и добавление данных
    let mut context = Context::new();
    context.insert("people", data);

    // Рендеринг шаблона в HTML
    let rendered = tera.render(template_name, &context)?;

    // Сохранение HTML в файл
    fs::write(output_path, rendered)?;

    println!("HTML has been written to {}", output_path);

    Ok(())
}