use std::error::Error;

use argh::FromArgs;
use serde_json::Value;

fn parse_currency(val: &str) -> Result<String, String> {
    Ok(val.trim().to_uppercase())
}

#[derive(FromArgs, Debug)]
/// Currency converter arguments
struct Args {
    /// source currency (default: USD)
    #[argh(
        option,
        short = 'f',
        default = "String::from(\"USD\")",
        from_str_fn(parse_currency)
    )]
    from: String,

    /// target currency
    #[argh(option, short = 't', from_str_fn(parse_currency))]
    to: String,

    /// amount to convert
    #[argh(positional)]
    amount: f64,
}

const URL: &str = "https://open.er-api.com/v6/latest";

fn get_rates(currency: &str) -> Result<Value, Box<dyn Error>> {
    let res: Value = ureq::get(format!("{URL}/{currency}"))
        .call()?
        .body_mut()
        .read_json()?;
    Ok(res)
}

fn main() {
    let args: Args = argh::from_env();
    let rates = get_rates(&args.from);

    match rates {
        Ok(rates) => {
            let rate: f64 = rates
                .get("rates")
                .unwrap()
                .get(&args.to)
                .unwrap()
                .as_f64()
                .unwrap();

            let res = args.amount * rate;
            println!("{:.2}{} = {:.2}{}", args.amount, args.from, res, args.to);
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}
