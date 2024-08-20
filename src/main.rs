use money::{Amount, Convert, Currency, Forex};
#[derive(Debug, Copy, Clone)]
pub struct USD;
#[derive(Debug, Copy, Clone)]
pub struct EUR;
#[derive(Debug, Copy, Clone)]
pub struct GPB;

impl Forex<EUR> for USD {
    const INTO_RATIO: f64 = 0.85; // Example conversion ratio
}

impl Forex<USD> for EUR {
    const INTO_RATIO: f64 = 1.18; // Example conversion ratio
}

impl Forex<USD> for GPB {
    const INTO_RATIO: f64 = 0.72; // Example conversion ratio
}

impl Currency for GPB {
    const SYMBLE: &'static str = "£";
    const CODE: &'static str = "GPB";
    const RITIO: u8 = 100;
}

impl Currency for USD {
    const SYMBLE: &'static str = "$";
    const CODE: &'static str = "USD";
    const RITIO: u8 = 100;
}

impl Currency for EUR {
    const SYMBLE: &'static str = "€";
    const CODE: &'static str = "EUR";
    const RITIO: u8 = 100;
}

fn main() {
    let usd0 = Amount::new(100_00, USD);
    let eur0 = Amount::new(100_00, EUR);

    //
    let amount_usd = Amount::<USD>::convert(eur0);
    let amount_eur: Amount<EUR> = Amount::convert(usd0);

    println!("USD amount {:#}", amount_usd);
    println!("EUR amount {:#}", amount_eur);
    println!("TOTAL {:#}", eur0 + usd0.to());

    let amount_eur: Amount<EUR> = usd0.to();
    println!("EUR amount {:#}", amount_eur);

    println!(" usd + usd  {}", amount_usd + usd0);
    println!(" eur + usd  {}", eur0 + usd0.to());
    let usd_amount = Amount::new(100, USD);
    let eur_amount: Amount<EUR> = usd_amount.to();

    println!("USD amount: {}", usd_amount);
    println!("EUR amount: {}", eur_amount);
    println!("EUR amount: {:#}", usd0);
}
