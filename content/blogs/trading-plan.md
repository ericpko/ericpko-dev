<!---
Jun 20, 2024
Investing
My 2024 Trading Plan
Trading, investing, and monthly income.
-->

Here's a link to a live version of my [2024 Trading Plan](https://typst.app/project/rNq1S7o_tWtU09J_jbZFyC).

# Trading Goals

-   Let the probabilities play out.
-   Be consistent, mechanical, and repeatable.
-   Generate 3% monthly income of the Net Liquidation (Net Liq) value.
-   Focus on the monthly realized P/L.

# Portfolio Greeks

1. **Delta** should never be more than 0.2% of the Net Liq.

# Risk Management

1. Always be at or below 50% of the available Buying Power (BP).
    - If BP is over 60%, **NO new trades**.
    - If BP hits 85%, take action to reduce BP:
        - Buy a 30 DTE Long Put.
        - Close a winning trade.
        - Close short Puts of a LT112.
        - Close BIL position.
2. Never risk more than 2% of the total portfolio on a single trade.
3. Never invest more than 60% of the max allowed BP on any one trading
   strategy (i.e. LT112s, Strangles, etc.).

# Core Trades

## /ES LT112 Campaign (Long Term 112)

-   We want to generate 2% per month from our LT112s.
-   Since it's a bi-monthly trade, we need to divide by 2, which gives us 1% per trade.
-   Our account size is starting at $230,000, and 1% of this is $2,300.
-   Next, we'll add 5% \* $2,300 = $115 to this to make up for the 5%
    we give up by exiting the Naked Puts at 95% profit.
-   This brings us to $2,300 + $115 = $2,415.
-   Since the /ES multiplier is 50, we need to divide this number by 50 to find out
    how much credit we want per trade: $2,415 / 50 = 48.3cr.
-   Thus, if we're trying to make 2% of our Net Liq from this strategy alone,
    then each trade we place should be close to 48.3cr, which is $2,415 in credit.
-   Since a typical LT112 receives around 22-24cr, we can add two of them per trade.

### Buying Power & Sizing

-   2 trades per month, laddering up to 8 trades.
    -   24cr * 50 multiplier * 2/month * 2 quantity/trade = $4,800/month
-   IBKR BP is around $30,000 per trade (with a qantity of 2).
-   Thus, this would require $18,200 * 2 qty/trade * 6 trades = $180,000 in BP.

This amount of BP would violate our Risk Management rule 1, but we are able to mitigate
our BP by adding Long Put hedges on every month.

### Trade Entry

1. Enter at 120 DTE.
2. Buy a 50 wide PDS starting at 25 delta for the Long Put.
3. Sell two 5-7 delta NPs.
4. Target credit is around $20-24cr.

### Trade Management

We manage the NPs and the PDS as two separate trades.

#### Naked Put Management

-   Take profit at 95%.
-   Stop loss at 3 times the initial credit (2X loss).
    -   If we receive a $1 credit and stop out at $3, then we've incurred at $2 debit (2X loss).
-   Or stop out at 2% of Net Liq -- which ever happens first.

#### Put Debit Spread (PDS) Management

-   Close any leg of the spread that's ITM by expiration to avoid being exercised or assigned.
-   Wait until close to expiration to close at or near full spread profit.
-   Close PDS at 300% profit. If this happens, you've probably hit the stop on the NPs.

## Strangles on Futures

!TODO
