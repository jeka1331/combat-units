use std::error::Error;

use boevoi::{
    read_naryad_from_toml, read_positions_from_toml, read_soldiers_from_toml,
    render_template_to_file, AssignedPosition, AssignedPositions, DutyGroupMember,
    DutyGroupMembers, FireGroupMembers, Soldier,
};
use chrono::{DateTime, Duration, Local};

fn main() -> Result<(), Box<dyn Error>> {
    let now: DateTime<Local> = Local::now();

    let duty_group_members: DutyGroupMembers = Vec::new();
    let mut assigned_positions: AssignedPositions = Vec::new();
    let fire_group_members: FireGroupMembers = Vec::new();
    let positions = read_positions_from_toml("data/positions.toml")?;
    let naryad = read_naryad_from_toml("data/naryad.toml")?;
    let naryad_for_shtab = naryad.clone();
    let naryad_for_kazarma = naryad.clone();
    let naryad_soldier_ids = naryad.get_only_values();

    let soldiers: Vec<Soldier> = read_soldiers_from_toml("data/soldiers.toml")?.soldiers;
    let shtab: Vec<Soldier> = naryad_for_shtab.get_shtab(soldiers.clone());
    let kazarma: Vec<Soldier> = naryad_for_kazarma.get_dnev(soldiers.clone());
    let soldiers: Vec<Soldier> = soldiers
        .iter()
        .filter(|&item| !naryad_soldier_ids.contains(&item.id)) // Условие фильтрации
        .cloned()
        .collect();

    let mut soldiers_for_assigned_positions = soldiers.clone();
    soldiers_for_assigned_positions.reverse();

    for position in positions.positions {
        println!("Soldier: {:?}", position);
        assigned_positions.push(AssignedPosition {
            position: position.clone(),
            members: {
                let members: Vec<Soldier> = Vec::new();

                let members = [
                    members,
                    soldiers_for_assigned_positions
                        .drain((soldiers_for_assigned_positions.len() - &position.member_count)..)
                        .collect(),
                ]
                .concat();
                let members: Vec<Soldier> = if position.name == "Штаб-наряд" {
                    shtab.clone()
                } else {
                    members
                };
                
                let members: Vec<Soldier> = if position.name == "Казарма" {
                    kazarma.clone()
                } else {
                    members
                };

                members
            },
        });
    }

    let filtered_kv: Vec<DutyGroupMember> = soldiers
        .iter()
        .filter(|&item| item.vzvod == "kv")
        .cloned()
        .take(5)
        .collect();
    let fire_group_members = [fire_group_members, filtered_kv].concat();

    let filtered_vnv: Vec<DutyGroupMember> = soldiers
        .iter()
        .filter(|&item| item.vzvod == "vnv")
        .cloned()
        .take(5)
        .collect();
    let duty_group_members = [duty_group_members, filtered_vnv].concat();

    let _ = render_template_to_file(
        "templates/**/*",
        "table.html",
        "output.html",
        assigned_positions,
        duty_group_members,
        fire_group_members,
        &now.format("%d.%m.%Y").to_string(),
        &(now + Duration::days(1)).format("%d.%m.%Y").to_string(),
    );

    Ok(())
}
