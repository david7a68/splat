---
splat_schema: 1
splat_id: 3c691f80-f94b-52ed-9fca-4da222a934ef
splat_kind: item
splat_key: dnd2024:items/weapon_1_2_or_3
splat_name: Weapon, +1, +2, or +3
splat_origin:
  key: dnd2024:reference/magic_item/weapon_1_2_or_3
splat_sources:
- title: SRD 5.2.1
  locator: pp. 253
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/weapon_1_2_or_3
  implementation: dnd2024:item_weapon_1_2_or_3
  parameters:
    variants:
    - rarity: uncommon
      attack_bonus: 1
      damage_bonus: 1
    - rarity: rare
      attack_bonus: 2
      damage_bonus: 2
    - rarity: very_rare
      attack_bonus: 3
      damage_bonus: 3
    bonus_scope: attacks_with_this_weapon
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_weapon_1_2_or_3
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
  outputs:
  - modifiers
  events:
  - equipment.state_changed
  capabilities:
  - equipment
  - derived_statistics
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: bc4c3de35db77d53579a17c82ef01bd6623fb5c71752afe6ae7f7d9143d32078
  source_path: builtin/srd-5.2.1/magic_item/weapon_1_2_or_3.md
  pages:
  - 253
  engine_gaps: &id001
  - dnd2024:item_weapon_1_2_or_3
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: b97e3cca172c90a995b718735ee372b903c039cd585565501b4d907b70b5523d
splat_clauses:
- key: source_rules
  text: '# Weapon, +1, +2, or +3 Weapon (Any Simple or Martial), Uncommon (+1), Rare (+2), or Very Rare
    (+3) You have a bonus to attack rolls and damage rolls made with this magic weapon. The bonus is determined
    by the weapon’s rarity.'
  sha256: a167d0b1e912d2f24364c80751e2242347f9da5b78e2ab5c51dd5527286dab60
  status: contract_specified
splat_requires_mechanics: *id001
---

# Weapon, +1, +2, or +3

<!-- SRD page 253 -->

Weapon (Any Simple or Martial), Uncommon (+1), Rare\
(+2), or Very Rare (+3)\
You have a bonus to attack rolls and damage rolls\
made with this magic weapon. The bonus is deter-\
mined by the weapon’s rarity.\

Original layout: [[../../srd-5.2.1/pages/253|page 253]].
