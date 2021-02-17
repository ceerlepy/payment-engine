use super::client::ClientAccount;

pub trait PaymentProcess{
    fn process(&mut self);
    fn get_client_account(&self) -> &ClientAccount;
}