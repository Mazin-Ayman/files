from requests import get
from argparse import ArgumentParser

URL = 'https://open.er-api.com/v6/latest'

######### CLI #########
parser = ArgumentParser("rc", "Currency convertor", "A cli tool for exchanging currencies")
parser.add_argument(
    "--from", "-f",
    dest="from_currency",
    default="USD",
    help="Currency to convert from (default: USD)"
)

parser.add_argument(
    "--to", "-t",
    dest="to_currency",
    required=True,
    help="Currency to convert to (REQUIRED)"
)

parser.add_argument(
    "amount",
    type=float,
    help="Amount to convert"
)
#######################

def get_rates(curr: str) -> dict | None:
    url = URL + f'/{curr}'
    res = get(url)
    if res.status_code == 200:
        return res.json()
    else:
        return None



def main() -> None:
    args = parser.parse_args()

    rates = get_rates(args.from_currency)
    if rates:
        rate = rates["rates"][args.to_currency.upper()]
        result = rate * args.amount
        print(f"{args.amount}{args.from_currency.upper()} = {result:.2f}{args.to_currency.upper()}")
    else:
        print("Failed to get exchange rate data.")

if __name__ == "__main__":
    main()
