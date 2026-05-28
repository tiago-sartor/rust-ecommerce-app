use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrazilianStates {
    AC, AL, AP, AM, BA, CE, DF, ES, GO, MA, MT, MS, MG, PA, PB, PR, PE, PI, RJ, RN, RS, RO, RR, SC, SP, SE, TO,
}

impl BrazilianStates {
    /// Returns the two-letter symbol (abbreviation)
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::AC => "AC", Self::AL => "AL", Self::AP => "AP", Self::AM => "AM",
            Self::BA => "BA", Self::CE => "CE", Self::DF => "DF", Self::ES => "ES",
            Self::GO => "GO", Self::MA => "MA", Self::MT => "MT", Self::MS => "MS",
            Self::MG => "MG", Self::PA => "PA", Self::PB => "PB", Self::PR => "PR",
            Self::PE => "PE", Self::PI => "PI", Self::RJ => "RJ", Self::RN => "RN",
            Self::RS => "RS", Self::RO => "RO", Self::RR => "RR", Self::SC => "SC",
            Self::SP => "SP", Self::SE => "SE", Self::TO => "TO",
        }
    }

    /// Returns an iterator over all variants
    pub fn all() -> &'static [BrazilianStates] {
        &[
            Self::AC, Self::AL, Self::AP, Self::AM, Self::BA, Self::CE, Self::DF, Self::ES,
            Self::GO, Self::MA, Self::MT, Self::MS, Self::MG, Self::PA, Self::PB, Self::PR,
            Self::PE, Self::PI, Self::RJ, Self::RN, Self::RS, Self::RO, Self::RR, Self::SC,
            Self::SP, Self::SE, Self::TO,
        ]
    }
}

impl fmt::Display for BrazilianStates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::AC => "Acre",
            Self::AL => "Alagoas",
            Self::AP => "Amapá",
            Self::AM => "Amazonas",
            Self::BA => "Bahia",
            Self::CE => "Ceará",
            Self::DF => "Distrito Federal",
            Self::ES => "Espírito Santo",
            Self::GO => "Goiás",
            Self::MA => "Maranhão",
            Self::MT => "Mato Grosso",
            Self::MS => "Mato Grosso do Sul",
            Self::MG => "Minas Gerais",
            Self::PA => "Pará",
            Self::PB => "Paraíba",
            Self::PR => "Paraná",
            Self::PE => "Pernambuco",
            Self::PI => "Piauí",
            Self::RJ => "Rio de Janeiro",
            Self::RN => "Rio Grande do Norte",
            Self::RS => "Rio Grande do Sul",
            Self::RO => "Rondônia",
            Self::RR => "Roraima",
            Self::SC => "Santa Catarina",
            Self::SP => "São Paulo",
            Self::SE => "Sergipe",
            Self::TO => "Tocantins",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseBrazilianStatesError;

impl fmt::Display for ParseBrazilianStatesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid Brazilian state symbol")
    }
}

impl std::error::Error for ParseBrazilianStatesError {}

impl FromStr for BrazilianStates {
    type Err = ParseBrazilianStatesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AC" => Ok(Self::AC),
            "AL" => Ok(Self::AL),
            "AP" => Ok(Self::AP),
            "AM" => Ok(Self::AM),
            "BA" => Ok(Self::BA),
            "CE" => Ok(Self::CE),
            "DF" => Ok(Self::DF),
            "ES" => Ok(Self::ES),
            "GO" => Ok(Self::GO),
            "MA" => Ok(Self::MA),
            "MT" => Ok(Self::MT),
            "MS" => Ok(Self::MS),
            "MG" => Ok(Self::MG),
            "PA" => Ok(Self::PA),
            "PB" => Ok(Self::PB),
            "PR" => Ok(Self::PR),
            "PE" => Ok(Self::PE),
            "PI" => Ok(Self::PI),
            "RJ" => Ok(Self::RJ),
            "RN" => Ok(Self::RN),
            "RS" => Ok(Self::RS),
            "RO" => Ok(Self::RO),
            "RR" => Ok(Self::RR),
            "SC" => Ok(Self::SC),
            "SP" => Ok(Self::SP),
            "SE" => Ok(Self::SE),
            "TO" => Ok(Self::TO),
            _ => Err(ParseBrazilianStatesError),
        }
    }
}
