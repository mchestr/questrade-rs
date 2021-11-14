use errors::QuestradeError;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

pub mod accounts;
pub mod auth;
pub mod client;
pub mod errors;
pub mod markets;
pub mod symbols;

pub use client::Client;
use url::Url;

#[derive(Debug, strum_macros::Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    Practice,
    Production,
}

impl Environment {
    fn host(&self) -> Result<Url, QuestradeError> {
        Ok(match self {
            Environment::Practice => Url::parse("https://practicelogin.questrade.com")?,
            Environment::Production => Url::parse("https://login.questrade.com")?,
        })
    }

    fn authorize_url(&self) -> Result<Url, QuestradeError> {
        Ok(self.host()?.join("/oauth2/authorize")?)
    }

    fn token_url(&self) -> Result<Url, QuestradeError> {
        Ok(self.host()?.join("/oauth2/token")?)
    }
}

#[derive(
    Debug, strum_macros::Display, strum_macros::EnumIter, Deserialize, Serialize, PartialEq, Clone,
)]
pub enum AccountType {
    Cash,
    FRESP,
    Margin,
    LIF,
    LIRA,
    LRIF,
    LRRSP,
    PRIF,
    RIF,
    SRIF,
    RESP,
    RRIF,
    RRSP,
    SRRSP,
    TFSA,
}

#[derive(
    Debug, strum_macros::Display, strum_macros::EnumIter, Deserialize, Serialize, PartialEq, Clone,
)]
pub enum AccountStatus {
    Active,
    #[strum(serialize = "Suspended (Closed)")]
    #[serde(rename = "Suspended (Closed)")]
    SuspendedClosed,
    #[strum(serialize = "Suspended (View Only)")]
    #[serde(rename = "Suspended (View Only)")]
    SuspendedViewOnly,
    #[strum(serialize = "Liquidate Only")]
    #[serde(rename = "Liquidate Only")]
    LiquidateOnly,
    Closed,
}

#[derive(
    Debug, strum_macros::Display, strum_macros::EnumIter, Deserialize, Serialize, PartialEq, Clone,
)]
pub enum ClientAccountType {
    Corporation,
    Family,
    #[strum(serialize = "Formal Trust")]
    #[serde(rename = "Formal Trust")]
    FormalTrust,
    Individual,
    #[strum(serialize = "Informal Trust")]
    #[serde(rename = "Informal Trust")]
    InformalTrust,
    Institution,
    #[strum(serialize = "Investment Club")]
    #[serde(rename = "Investment Club")]
    InvestmentClub,
    Joint,
    #[strum(serialize = "Joint and Informal Trust")]
    #[serde(rename = "Joint and Informal Trust")]
    JointAndInformalTrust,
    Partnership,
    #[strum(serialize = "Sole Proprietorship")]
    #[serde(rename = "Sole Proprietorship")]
    SoleProprietorship,
}

#[derive(
    Debug, strum_macros::Display, strum_macros::EnumIter, Deserialize, Serialize, PartialEq, Clone,
)]
pub enum Currency {
    CAD,
    USD,
}

#[derive(
    Debug, strum_macros::EnumIter, Deserialize_enum_str, Serialize_enum_str, PartialEq, Clone,
)]
pub enum ActivityType {
    Interest,
    Deposits,
    Trades,
    Dividends,
    #[serde(rename = "FX conversion")]
    #[strum(serialize = "FX conversion")]
    FXConversion,
    #[serde(other)]
    Other(String),
}

#[derive(
    Debug, strum_macros::Display, strum_macros::EnumIter, Deserialize, Serialize, PartialEq, Clone,
)]
pub enum StateFilter {
    All,
    Open,
    Closed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_type_display_works() {
        <AccountType as strum::IntoEnumIterator>::iter().for_each(|t| {
            let expected_string = match t {
                AccountType::Cash => "Cash",
                AccountType::FRESP => "FRESP",
                AccountType::Margin => "Margin",
                AccountType::LIF => "LIF",
                AccountType::LIRA => "LIRA",
                AccountType::LRIF => "LRIF",
                AccountType::LRRSP => "LRRSP",
                AccountType::PRIF => "PRIF",
                AccountType::RIF => "RIF",
                AccountType::SRIF => "SRIF",
                AccountType::RESP => "RESP",
                AccountType::RRIF => "RRIF",
                AccountType::RRSP => "RRSP",
                AccountType::SRRSP => "SRRSP",
                AccountType::TFSA => "TFSA",
            };
            assert_eq!(expected_string, format!("{}", t));
        });
    }

    #[test]
    fn account_status_display_works() {
        <AccountStatus as strum::IntoEnumIterator>::iter().for_each(|s| {
            let expected_string = match s {
                AccountStatus::Active => "Active",
                AccountStatus::SuspendedClosed => "Suspended (Closed)",
                AccountStatus::SuspendedViewOnly => "Suspended (View Only)",
                AccountStatus::LiquidateOnly => "Liquidate Only",
                AccountStatus::Closed => "Closed",
            };
            assert_eq!(expected_string, format!("{}", s));
        });
    }

    #[test]
    fn client_account_type_display_works() {
        <ClientAccountType as strum::IntoEnumIterator>::iter().for_each(|t| {
            let expected_string = match t {
                ClientAccountType::Corporation => "Corporation",
                ClientAccountType::Family => "Family",
                ClientAccountType::FormalTrust => "Formal Trust",
                ClientAccountType::Individual => "Individual",
                ClientAccountType::InformalTrust => "Informal Trust",
                ClientAccountType::Institution => "Institution",
                ClientAccountType::InvestmentClub => "Investment Club",
                ClientAccountType::Joint => "Joint",
                ClientAccountType::JointAndInformalTrust => "Joint and Informal Trust",
                ClientAccountType::Partnership => "Partnership",
                ClientAccountType::SoleProprietorship => "Sole Proprietorship",
            };
            assert_eq!(expected_string, format!("{}", t));
        })
    }

    #[test]
    fn currency_display_works() {
        <Currency as strum::IntoEnumIterator>::iter().for_each(|t| {
            let expected_string = match t {
                Currency::CAD => "CAD",
                Currency::USD => "USD",
            };
            assert_eq!(expected_string, format!("{}", t));
        })
    }

    #[test]
    fn state_filter_display_works() {
        <StateFilter as strum::IntoEnumIterator>::iter().for_each(|f| {
            let expected_string = match f {
                StateFilter::All => "All",
                StateFilter::Open => "Open",
                StateFilter::Closed => "Closed",
            };
            assert_eq!(expected_string, format!("{}", f));
        })
    }
}
