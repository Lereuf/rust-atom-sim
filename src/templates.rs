use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Atome
{
    pub nom_atome: String,
    pub nb_neutrons: i32,
    pub nb_protons: i32,
    pub nb_electrons: i32,
}