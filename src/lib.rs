use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use tera::{Context, Tera};

pub type AssignedPositions = Vec<AssignedPosition>;
pub type FireGroupMembers = Vec<FireGroupMember>;
pub type DutyGroupMembers = Vec<DutyGroupMember>;
pub type Soldiers = Vec<Soldier>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AssignedPosition {
    pub position: Position,
    pub members: Vec<Soldier>,
}

impl AssignedPosition {
    pub fn add_member(&mut self, member: Soldier) {
        let member_to_add = vec![member.clone()];
        let member_to_add2 = self.members.clone().clone();
        let combined = vec![member_to_add, member_to_add2].concat();
        self.members = combined;
    }
}

pub type DutyGroupMember = Soldier;
pub type FireGroupMember = Soldier;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Soldier {
    pub id: usize,
    pub rank: String,
    pub fio: String,
    pub vzvod: String,
}

#[derive(Debug, Deserialize)]
pub struct SoldiersList {
    pub soldiers: Vec<Soldier>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
    pub name: String,
    pub ammo: String,
    pub member_count: usize,
    pub vzvod_priority: String,
}

#[derive(Debug, Deserialize)]
pub struct PositionsList {
    pub positions: Vec<Position>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Naryad {
    kpp1_1: usize,
    kpp1_2: usize,
    kpp2_1: usize,
    kpp3_1: usize,
    kpp3_2: usize,
    kpp4_1: usize,
    kpp4_2: usize,
    shtab_pom: usize,
    shtab_dezh: usize,
    dnev_1: usize,
    dnev_2: usize,
    dezh: usize,
}

impl Naryad {
    pub fn get_only_values(self) -> Vec<usize> {
        vec![
            self.kpp1_1,
            self.kpp1_2,
            self.kpp2_1,
            self.kpp3_1,
            self.kpp3_2,
            self.kpp4_1,
            self.kpp4_2,
            self.shtab_pom,
            self.shtab_dezh,
            self.dnev_1,
            self.dnev_2,
            self.dezh,
        ]
    }

    pub fn get_shtab(self, soldiers: Soldiers) -> Vec<Soldier> {
        soldiers
            .clone()
            .iter()
            .filter(|&item| item.id == self.shtab_pom || item.id == self.shtab_dezh) // Условие фильтрации
            .cloned()
            .collect()
    }

    pub fn get_dnev(self, soldiers: Soldiers) -> Vec<Soldier> {
        soldiers
            .clone()
            .iter()
            .filter(|&item| item.id == self.dnev_1 || item.id == self.dnev_2) // Условие фильтрации
            .cloned()
            .collect()
    }
}

pub fn read_soldiers_from_toml(file_path: &str) -> Result<SoldiersList, Box<dyn Error>> {
    let toml_str = fs::read_to_string(file_path)?;
    let result: SoldiersList = toml::de::from_str(&toml_str)?;
    Ok(result)
}

pub fn read_naryad_from_toml(file_path: &str) -> Result<Naryad, Box<dyn Error>> {
    let toml_str = fs::read_to_string(file_path)?;
    let result: Naryad = toml::de::from_str(&toml_str)?;
    Ok(result)
}

pub fn read_positions_from_toml(file_path: &str) -> Result<PositionsList, Box<dyn Error>> {
    let toml_str = fs::read_to_string(file_path)?;
    let result: PositionsList = toml::de::from_str(&toml_str)?;
    Ok(result)
}

pub fn render_template_to_file(
    template_path: &str,
    template_name: &str,
    output_path: &str,
    assigned_positions: AssignedPositions,
    duty_group_members: DutyGroupMembers,
    fire_group_members: FireGroupMembers,
    creation_date: &str,
    next_day: &str,
) -> Result<(), Box<dyn Error>> {
    let tera = Tera::new(template_path)?;

    let mut context = Context::new();
    context.insert("duty_group_members", &duty_group_members);
    context.insert("assigned_positions", &assigned_positions);
    context.insert("fire_group_members", &fire_group_members);
    context.insert("creation_date", creation_date);
    context.insert("next_day", next_day);

    let rendered = tera
        .render(template_name, &context)
        .expect("Файл не прошел рендер");

    fs::write(output_path, rendered).expect("Файл не записан");

    println!("HTML has been written to {}", output_path);

    Ok(())
}
