# Input-contract format

This contract input is a simplified and streamlined adaptation of the one used in `rust-dlc`

Every benchmark run expects a DLC “contract-input” file in **JSON**.  
The parser deserialises it into `ContractInput`, validates a few sanity rules,
then converts it to the internal `ParsedContract` structure.

Some fields are not actively used in framework, but we left those, that might potentialy be beneficial in future experimentations. They are striked through in list.

numerical_contract_input.json ← reference file used in all benchmarks  
└── contractInfo  
&#8195;&#8195;    ├── contractDescriptor  
&#8195;&#8195;    │  &#8195; └── payoutIntervals ← outcome-based payout ranges   
&#8195;&#8195;    └── oracle ← oracle announcement parameters


## Top-level fields

| Field              | Type     | Meaning                                                        |
|--------------------|----------|----------------------------------------------------------------|
| `offerCollateral`  | u64      | Satoshis pledged by the offerer (Alice)                        |
| `acceptCollateral` | u64      | Satoshis pledged by the accepter (Bob)                         |
| ~~`feeRate`~~      | ~~u64~~  | ~~Sats/vB used for the (simulated) funding/CET transactions~~  |
| `contractInfo`     | struct   | Descriptor and oracle information (see below)                  |

### `contractInfo.contractDescriptor`

`payoutIntervals` is an **array of consecutive intervals**.  
Each interval contains exactly two `payoutPoints`.  
A `payoutPoint` specifies:

| Field           | Type | Comment                                        |
|-----------------|------|------------------------------------------------|
| `eventOutcome`  | u32  | Outcome value (BTC price, etc.)                |
| `outcomePayout` | u64  | Satoshis paid to the _offerer_ at that outcome |

Linear interpolation between the two points of each interval yields every
individual outcome/payout pair used later by the framework.

### `contractInfo.oracle`

| Field            | Type     | Comment                                        |
|------------------|----------|------------------------------------------------|
| ~~`publicKey`~~  | ~~hex~~  | ~~Oracle’s compressed secp256k1 public key~~   |
| ~~`eventId`~~    | ~~str~~  | ~~Event identifier supplied by the oracle~~    |
| `nbDigits`       | u8       | Number of binary digits representing outcome   |

---

## Validation rules

The loader rejects the input if any rule fails.

| # | Rule                                                                                               |
|---|----------------------------------------------------------------------------------------------------|
| 1 | Intervals must be **continuous in outcomes** (end of one interval = start of the next).            |
| 2 | The very first `eventOutcome` must be **0**.                                                       |
| 3 | The last `eventOutcome` must be **2<sup>nbDigits</sup> − 1**.                                      |
| 4 | `outcomePayout` ≤ `offerCollateral` + `acceptCollateral`.                                          |
| 5 | `outcomePayout` is non-negative (enforced by unsigned type).                                       |
| 6 | `offerCollateral` and `acceptCollateral` are positive integers (enforced by unsigned type).        |
| 7 | `feeRate` ≤ 25 × 250 sat/vB (anything higher is considered invalid).                               |
| 8 | Each interval contains **exactly two** payout points.                                              |
| 9 | At least one interval must be present.                                                             |
|10 | File must not be empty; all required fields must exist. (⚠️changed with Kixunil PR - look later )  |
|11 | `nbDigits` in the file must equal the code constant `NB_DIGITS`.                                   |

Invalid contracts are rejected gracefully with a descriptive error instead of a panic.

---

## Test contract inputs

The folder also contains 10 negative-test contracts, each violating exactly one rule:

| File                               | Violated rule |
|------------------------------------|---------------|
| `noncontinuous_input.json`         | 1             |
| `nonzero_first_point_input.json`   | 2             |
| `last_outcome_not_final_input.json`| 3             |
| `invalid_payout_level_input.json`  | 4             |
| `negative_payout_input.json`       | 5             |
| `invalid_collateral_input.json`    | 6             |
| `excessive_feerate_input.json`     | 7             |
| `invalid_interval_points_input.json`| 8            |
| `no_intervals_input.json`          | 9             |
| `empty_contract_input.json`        | 10            |

Use these files when running `cargo test` to verify the parser rejects malformed
contracts as expected.
