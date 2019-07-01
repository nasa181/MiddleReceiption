pub struct CreditCard<'a>{
    pub nameOnCard: &'a str,
    pub cardNumber: &'a str,
    pub cvv: i16,
    pub expMonth: u8,
    pub expYear: i16
}

impl<'a> CreditCard<'a> {
    pub fn new(nameOnCard: &'a str, cardNumber: &'a str, cvv: i16, expMonth: u8, expYear: i16) -> CreditCard<'a> {
        CreditCard{nameOnCard: nameOnCard, cardNumber: cardNumber, cvv: cvv, expMonth: expMonth, expYear: expYear}
    }
}