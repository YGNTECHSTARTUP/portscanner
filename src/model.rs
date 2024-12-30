use serde::Deserialize;
#[derive(Debug, Clone)]
pub struct Domain {
    pub dom_name: String,
    pub ports: Vec<Ports>,
}

#[derive(Debug, Clone)]
pub struct Ports {
    pub port_name: String,
    pub is_open: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Scaentry {
    pub name_value: String,
}
