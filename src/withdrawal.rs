use super::payment_process::PaymentProcess;
use super::client::ClientAccount;

pub struct WithDrawal<'a>{
    pub client_account: &'a mut ClientAccount,
    pub amount: f64
}

impl<'a> PaymentProcess for WithDrawal<'a>{
    fn process(&mut self){
        if self.client_account.available >= self.amount {
            self.client_account.available -= self.amount;
            self.client_account.total -= self.amount;
        }else{
            println!("withdrawal operation --> insufficient available funds {}, amount: {}", self.client_account.available, self.amount);
        }
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
    fn test_withdrawal() {
       
        let pr : &mut dyn PaymentProcess = &mut WithDrawal{client_account: 
            &mut ClientAccount{client: 1u16, available: 3.0, held: 0.0, total: 3.0, locked: false}, amount: 1.0};
        pr.process();
        assert_eq!(pr.get_client_account().total, 2.0);
        assert_eq!(pr.get_client_account().available, 2.0);
    }

    #[test]
    fn test_withdrawal_when_available_less_than_amount() {
       
        let pr : &mut dyn PaymentProcess = &mut WithDrawal{client_account: 
            &mut ClientAccount{client: 1u16, available: 0.9, held: 0.0, total: 3.0, locked: false}, amount: 1.0};
        pr.process();
        assert_eq!(pr.get_client_account().total, 3.0);
        assert_eq!(pr.get_client_account().available, 0.9);
    }
}