use std::error::Error;

use boevoi::{
    read_positions_from_toml, read_soldiers_from_toml, render_template_to_file, AssignedPosition,
    AssignedPositions, DutyGroupMember, DutyGroupMembers, FireGroupMember, FireGroupMembers,
    Position, Soldier,
};
use chrono::{DateTime, Duration, Local};

fn main() -> Result<(), Box<dyn Error>> {
    let now: DateTime<Local> = Local::now();

    // Чтение с файлов
    let soldiers = read_soldiers_from_toml("data/soldiers.toml")?;
    for value in &soldiers.soldiers {
        println!("Soldier: {:?}", value);
    }

    let positions = read_positions_from_toml("data/positions.toml")?;
    for value in &positions.positions {
        println!("Soldier: {:?}", value);
    }

    let mut duty_group_members: DutyGroupMembers = Vec::new();
    let mut assigned_positions: AssignedPositions = Vec::new();
    let mut fire_group_members: FireGroupMembers = Vec::new();

    fire_group_members.push(FireGroupMember {
        rank: "рядовой".to_string(),
        fio: "Kural".to_string(),
        vzvod: "VNV".to_string(),
    });

    assigned_positions.push(AssignedPosition {
        position: Position {
            name: "Штаб".to_string(),
            ammo: "БКАР".to_string(),
            member_count: 1,
            vzvod_priority: "kv".to_string(),
        },
        member: Soldier {
            rank: "рядовой".to_string(),
            fio: "Kural".to_string(),
            vzvod: "VNV".to_string(),
        },
    });

    duty_group_members.push(DutyGroupMember {
        rank: "рядовой".to_string(),
        fio: "Kural".to_string(),
        vzvod: "VNV".to_string(),
    });

    // Вызов функции для рендеринга и сохранения
    let _ = render_template_to_file(
        "templates/**/*", // Путь к шаблонам
        "table.html",     // Имя шаблона
        "output.html",
        assigned_positions, // Путь к файлу для вывода
        duty_group_members,
        fire_group_members,
        &now.format("%d.%m.%Y").to_string(),
        &(now + Duration::days(1)).format("%d.%m.%Y").to_string(),
    );

    // let mut assigned_positions: Vec<(Position, Vec<Soldier>)> = Vec::new();
    Ok(())
}
