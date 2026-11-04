---
splat_schema: 2
splat_id: 1d820f6f-d00b-5e31-a018-506859f95280
splat_kind: rule
splat_key: dnd2024:rules/coins
splat_name: Coins
splat_origin: dnd2024:reference/pages/089
splat_sources:
- SRD 5.2.1, pp. 89, https://www.dndbeyond.com/srd
---

# Coins

## Source text

<!-- SRD page 89 -->

Characters often find coins on their adventures and can spend those coins in shops, inns, and other businesses. Coins come in different denominations based on the relative worth of their material. The Coin Values table lists coins and how much they’re worth relative to the Gold Piece, which is the game’s main coin. For example, 100 Copper Pieces are worth 1 Gold Piece.

A coin weighs about a third of an ounce, so fifty coins weigh a pound.

**Coin Values**

| Coin | Value in GP |
| --- | --- |
| Copper Piece (CP) | 1/100 |
| Silver Piece (SP) | 1/10 |
| Electrum Piece (EP) | 1/2 |
| Gold Piece (GP) | 1 |
| Platinum Piece (PP) | 10 |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/coins ` |
| ` implementation ` | ` dnd2024:rules_coins ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` coins_per_pound ` | ` 50 ` |

#### Gold piece value

<!-- splat:values path=splat_rule/parameters/gold_piece_value -->

| Field | Value |
| --- | --- |
| ` copper ` | ` 0.01 ` |
| ` silver ` | ` 0.1 ` |
| ` electrum ` | ` 0.5 ` |
| ` gold ` | ` 1 ` |
| ` platinum ` | ` 10 ` |

#### Approximate ounces per coin

<!-- splat:values path=splat_rule/parameters/approximate_ounces_per_coin -->

| Field | Value |
| --- | --- |
| ` numerator ` | ` 1 ` |
| ` denominator ` | ` 3 ` |
