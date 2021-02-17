use super::payment_process::PaymentProcess;
use super::client::ClientAccount;

pub struct Dispute<'a>{
    pub client_account: &'a mut ClientAccount,
    pub amount: f64
}

impl<'a> PaymentProcess for Dispute<'a>{
    fn process(&mut self){
        self.client_account.available -= self.amount;
        self.client_account.held += self.amount;
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
    fn test_dispute() {
       
        let pr : &mut dyn PaymentProcess = &mut Dispute{client_account: 
            &mut ClientAccount{client: 1u16, available: 3.0, held: 0.0, total: 3.0, locked: false}, amount: 1.0};
        pr.process();
        assert_eq!(pr.get_client_account().available, 2.0);
        assert_eq!(pr.get_client_account().held, 1.0);
    }
}