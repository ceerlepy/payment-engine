use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub transaction_type: String,
    pub client: u16,
    pub tx: u32,
    #[serde(deserialize_with = "default_if_empty")]
    pub amount: f64
}

fn default_if_empty<'de, D, T>(de: D) -> Result<T, D::Error>
where D: serde::Deserializer<'de>, T: serde::Deserialize<'de> + Default,
{
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_else(|| T::default()))
}

#[derive(Debug)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback
}

impl TransactionType {

    pub fn from_string(input: String) -> Result<Self, ()> {
        match input.to_lowercase().as_str() {
            "deposit" => Ok(TransactionType::Deposit),
            "withdrawal" => Ok(TransactionType::Withdrawal),
            "dispute"  => Ok(TransactionType::Dispute),
            "resolve" => Ok(TransactionType::Resolve),
            "chargeback" => Ok(TransactionType::Chargeback),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_transaction() {
        assert!(matches!(TransactionType::from_string(String::from("Resolve")).unwrap(), TransactionType::Resolve));
        assert!(matches!(TransactionType::from_string(String::from("withdrawal")).unwrap(), TransactionType::Withdrawal));
        assert!(matches!(TransactionType::from_string(String::from("dispute")).unwrap(), TransactionType::Dispute));
        assert!(matches!(TransactionType::from_string(String::from("deposit")).unwrap(), TransactionType::Deposit));
        assert!(matches!(TransactionType::from_string(String::from("chargeback")).unwrap(), TransactionType::Chargeback));
    }
}