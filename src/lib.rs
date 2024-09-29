use serde::{Deserialize, Serialize};
use tera::{Tera, Context};
use std::error::Error;
use std::fs;




pub type AssignedPositions = Vec<AssignedPosition>;
pub type FireGroupMembers = Vec<FireGroupMember>;
pub type DutyGroupMembers = Vec<DutyGroupMember>;

#[derive(Debug, Deserialize, Serialize)]
pub struct AssignedPosition {
    pub position: Position,
    pub member: Soldier,
}



pub type DutyGroupMember = Soldier;
pub type FireGroupMember = Soldier;

#[derive(Debug, Deserialize, Serialize)]
pub struct Soldier {
    pub rank: String,
    pub fio: String,
    pub vzvod: String,
}

#[derive(Debug, Deserialize)]
pub struct SoldiersList {
    pub soldiers: Vec<Soldier>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub name: String,
    pub ammo: String,
    pub member_count: isize,
    pub vzvod_priority: String,
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
    assigned_positions: AssignedPositions,
    duty_group_members: DutyGroupMembers,
    fire_group_members: FireGroupMembers,
    creation_date: &str,
    next_day: &str

) -> Result<(), Box<dyn Error>> {
    // Инициализация шаблонизатора Tera и загрузка шаблонов
    let tera = Tera::new(template_path)?;

    // Создание контекста и добавление данных
    let mut context = Context::new();
    context.insert("duty_group_members", &duty_group_members);
    context.insert("assigned_positions", &assigned_positions);
    context.insert("fire_group_members", &fire_group_members);
    context.insert("creation_date", creation_date);
    context.insert("next_day", next_day);

    // Рендеринг шаблона в HTML
    let rendered = tera.render(template_name, &context).expect("Файл не прошел рендер");

    // Сохранение HTML в файл
    fs::write(output_path, rendered).expect("Файл не записан");

    println!("HTML has been written to {}", output_path);

    Ok(())
}