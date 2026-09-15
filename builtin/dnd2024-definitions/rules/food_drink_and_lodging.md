---
splat_schema: 1
splat_id: 6e633ecb-2f5d-54de-9745-a598e3d5217b
splat_kind: rule
splat_key: dnd2024:rules/food_drink_and_lodging
splat_name: Food, Drink, and Lodging
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101, 102
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/food_drink_and_lodging
  parameters:
    daily_meals_and_lodging_included_in_lifestyle: true
    food_gp:
      ale_mug: 0.04
      bread_loaf: 0.02
      cheese_wedge: 0.1
    lodging_gp_per_day:
      squalid: 0.07
      poor: 0.1
      modest: 0.5
      comfortable: 0.8
      wealthy: 2
      aristocratic: 4
    meal_gp:
      squalid: 0.01
      poor: 0.02
      modest: 0.1
      comfortable: 0.2
      wealthy: 0.3
      aristocratic: 0.6
    wine_gp_per_bottle:
      common: 0.2
      fine: 10
  implementation: dnd2024:rules_food_drink_and_lodging
splat_contracts:
- id: dnd2024:rules_food_drink_and_lodging
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - food_and_lodging_prices
  events: []
  capabilities:
  - currency
  - downtime
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 101
  - 102
  engine_gaps: &id001
  - dnd2024:rules_food_drink_and_lodging
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 8bfe788e7302732adcfd72bdda9fd145f99f442be0fbc6e2a95d7dd7c5eb0f7e
  parent_name: Food, Drink, and Lodging
  line_span:
    first: 8820
    count: 18
splat_clauses:
- key: source_rules
  text: '# Food, Drink, and Lodging The Food, Drink, and Lodging table gives prices for food and a single
    night’s lodging. Prices for daily lodging and meals are included in your lifestyle’s expenses. Food,
    Drink, and Lodging Item Cost Item Cost Ale (mug) 4 CP Comfortable 8 SP Bread (loaf) 2 CP Wealthy 2
    GP Cheese (wedge) 1 SP Aristocratic 4 GP Inn Stay per Day Meal Squalid 7 CP Squalid 1 CP Poor 1 SP
    Poor 2 CP Modest 5 SP Modest 1 SP Item Cost Item Cost Comfortable 2 SP Wine (bottle) Wealthy 3 SP
    Common 2 SP Aristocratic 6 SP Fine 10 GP'
  sha256: 2cb3c05bc644a2bf26a7c1697c1a650a66edb6a9183030063b9a802d626de12a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Food, Drink, and Lodging

<!-- SRD page 101 -->

The Food, Drink, and Lodging table gives prices for\
food and a single night’s lodging. Prices for daily\
lodging and meals are included in your lifestyle’s\
expenses.\
Food, Drink, and Lodging\
Item Cost Item Cost\
Ale (mug) 4 CP Comfortable 8 SP\
Bread (loaf) 2 CP Wealthy 2 GP\
Cheese (wedge) 1 SP Aristocratic 4 GP\
Inn Stay per Day Meal\
Squalid 7 CP Squalid 1 CP\
Poor 1 SP Poor 2 CP\
Modest 5 SP Modest 1 SP\

<!-- SRD page 102 -->

Item Cost Item Cost\
Comfortable 2 SP Wine (bottle)\
Wealthy 3 SP Common 2 SP\
Aristocratic 6 SP Fine 10 GP\
