---
splat_schema: 1
splat_id: 1d820f6f-d00b-5e31-a018-506859f95280
splat_kind: rule
splat_key: dnd2024:rules/coins
splat_name: Coins
splat_origin:
  key: dnd2024:reference/pages/089
splat_sources:
- title: SRD 5.2.1
  locator: pp. 89
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/coins
  parameters:
    gold_piece_value:
      copper: 0.01
      silver: 0.1
      electrum: 0.5
      gold: 1
      platinum: 10
    coins_per_pound: 50
    approximate_ounces_per_coin:
      numerator: 1
      denominator: 3
  implementation: dnd2024:rules_coins
splat_contracts:
- id: dnd2024:rules_coins
  inputs:
    coins:
      type: resolution
      required: true
  outputs:
  - value_gp
  - weight_lb
  events: []
  capabilities:
  - currency
  - inventory
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 89
  engine_gaps: &id001
  - dnd2024:rules_coins
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 7130eadc7c50279b8cfac6c8c91c515b5f2df877decdb32bc61d906497ea03e6
  parent_name: Coins
  line_span:
    first: 7641
    count: 18
splat_clauses:
- key: source_rules
  text: '# Coins Characters often find coins on their adventures and can spend those coins in shops, inns,
    and other businesses. Coins come in different denominations based on the relative worth of their material.
    The Coin Values table lists coins and how much they’re worth relative to the Gold Piece, which is
    the game’s main coin. For example, 100 Copper Pieces are worth 1 Gold Piece. A coin weighs about a
    third of an ounce, so fifty coins weigh a pound. Coin Values Coin Value in GP Copper Piece (CP) 1/100
    Silver Piece (SP) 1/10 Electrum Piece (EP) 1/2 Gold Piece (GP) 1 Platinum Piece (PP) 10'
  sha256: bfdc4ab4e278333fd32f9b4943b97659cecd3a8ba82ad04dcce7a87c2df696fd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Coins

<!-- SRD page 89 -->

Characters often find coins on their adventures\
and can spend those coins in shops, inns, and other\
businesses. Coins come in different denominations\
based on the relative worth of their material. The\
Coin Values table lists coins and how much they’re\
worth relative to the Gold Piece, which is the game’s\
main coin. For example, 100 Copper Pieces are\
worth 1 Gold Piece.\
A coin weighs about a third of an ounce, so fifty\
coins weigh a pound.\
Coin Values\
Coin Value in GP\
Copper Piece (CP) 1/100\
Silver Piece (SP) 1/10\
Electrum Piece (EP) 1/2\
Gold Piece (GP) 1\
Platinum Piece (PP) 10\
