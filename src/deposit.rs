use super::payment_process::PaymentProcess;
use super::client::ClientAccount;

pub struct Deposit<'a>{
    pub client_account: &'a mut ClientAccount,
    pub amount: f64
}

impl<'a> PaymentProcess for Deposit<'a>{
    fn process(&mut self){
        self.client_account.available += self.amount;
        self.client_account.total += self.amount;
    }

    fn get_client_account(&self) -> &ClientAccount{
        &*self.client_account
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_deposit() {
       
        let pr : &mut dyn PaymentProcess = &mut Deposit{client_account: &mut ClientAccount::new(1u16), amount: 1.0};
        pr.process();
        assert_eq!(pr.get_client_account().total, 1.0);
    }
}