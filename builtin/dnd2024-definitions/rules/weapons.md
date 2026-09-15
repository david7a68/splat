---
splat_schema: 1
splat_id: fc0703ea-5fa2-5f10-8ebd-bfa9f9873944
splat_kind: rule
splat_key: dnd2024:rules/weapons
splat_name: Weapons
splat_origin:
  key: dnd2024:reference/pages/089
splat_sources:
- title: SRD 5.2.1
  locator: pp. 89
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons
  parameters:
    weapon_categories:
    - simple
    - martial
    weapon_kinds:
    - melee
    - ranged
    base_melee_reach_feet: 5
    damage_on_hit_source: weapon_damage_and_type
    properties_source: weapon_definition
    mastery:
      requires_feature_unlock: true
    selling:
      equipment_fraction_of_purchase_cost: 0.5
      trade_goods_and_valuables_fraction: 1
      magic_item_prices: magic_item_rules
  implementation: dnd2024:rule_weapons
splat_contracts:
- id: dnd2024:rule_weapons
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - item_effects
  events: []
  capabilities:
  - equipment
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 89
  engine_gaps: &id001
  - dnd2024:rule_weapons
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: e97cd3a5cd509589dd798d8bd3cb36e5618013a91d149259a550acba59ec40b4
  parent_name: Weapons
  line_span:
    first: 7659
    count: 27
splat_clauses:
- key: source_rules
  text: '# Weapons The Weapons table in this section shows the game’s main weapons. The table lists the
    cost and weight of each weapon, as well as the following details: Category. Every weapon falls into
    a category: Simple or Martial. Weapon proficiencies are usually tied to one of these categories. For
    example, you might have proficiency with Simple weapons. Melee or Ranged. A weapon is classified as
    either Melee or Ranged. A Melee weapon is used to attack a target within 5 feet, whereas a Ranged
    weapon is used to attack at a greater distance. Damage. The table lists the amount of damage a weapon
    deals when an attacker hits with it as well as the type of that damage. Properties. Any properties
    a weapon has are listed in the Properties column. Each property is defined in the “Properties” section.
    Mastery. Each weapon has a mastery property, which is defined in the “Mastery Properties” section.
    To use that property, you must have a feature that lets you use it. sEllinG EquipMEnt Equipment fetches
    half its cost when sold. In contrast, trade goods and valuables—like gems and art objects—retain their
    full value in the marketplace. “Magic Items” has prices for magic items.'
  sha256: 9fdd56366ef8834f5ebcc02e56a2799fd481cd71d91b0b97b667c5026eb5a8bd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Weapons

<!-- SRD page 89 -->

The Weapons table in this section shows the game’s\
main weapons. The table lists the cost and weight of\
each weapon, as well as the following details:\
Category. Every weapon falls into a category: Sim-\
ple or Martial. Weapon proficiencies are usually\
tied to one of these categories. For example, you\
might have proficiency with Simple weapons.\
Melee or Ranged. A weapon is classified as either\
Melee or Ranged. A Melee weapon is used to at-\
tack a target within 5 feet, whereas a Ranged\
weapon is used to attack at a greater distance.\
Damage. The table lists the amount of damage a\
weapon deals when an attacker hits with it as well\
as the type of that damage.\
Properties. Any properties a weapon has are listed\
in the Properties column. Each property is de-\
fined in the “Properties” section.\
Mastery. Each weapon has a mastery property,\
which is defined in the “Mastery Properties” sec-\
tion. To use that property, you must have a feature\
that lets you use it.\
sEllinG EquipMEnt\
Equipment fetches half its cost when sold. In con-\
trast, trade goods and valuables—like gems and art\
objects—retain their full value in the marketplace.\
“Magic Items” has prices for magic items.\
