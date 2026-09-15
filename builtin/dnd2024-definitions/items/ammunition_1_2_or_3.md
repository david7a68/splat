---
splat_schema: 1
splat_id: a96e093e-8791-5622-8bea-c073ba0b175c
splat_kind: item
splat_key: dnd2024:items/ammunition_1_2_or_3
splat_name: Ammunition, +1, +2, or +3
splat_origin:
  key: dnd2024:reference/magic_item/ammunition_1_2_or_3
splat_sources:
- title: SRD 5.2.1
  locator: pp. 209
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/ammunition_1_2_or_3
  implementation: dnd2024:item_ammunition_1_2_or_3
  parameters:
    variants:
    - rarity: uncommon
      attack_roll_bonus: 1
      damage_roll_bonus: 1
      scope: attacks_with_source_item
    - rarity: rare
      attack_roll_bonus: 2
      damage_roll_bonus: 2
      scope: attacks_with_source_item
    - rarity: very_rare
      attack_roll_bonus: 3
      damage_roll_bonus: 3
      scope: attacks_with_source_item
    on_hit:
      magical: false
    typical_lot_sizes:
    - 10
    - 20
    valuation:
      pieces: 10
      equivalent_category: potion
      rarity: same
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
splat_contracts:
- id: dnd2024:item_ammunition_1_2_or_3
  inputs:
    actor:
      type: participant
      required: true
    source_item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choice:
      required: false
      type: choice
    event:
      required: false
      type: event
    campaign:
      required: false
      type: campaign
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - attack.roll_pending
  - attack.hit
  capabilities:
  - inventory
  - effects
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 8aea3061c2fc460c6a6693764fb663837b71ccce7a6952a87ce5099af62cf974
  source_path: builtin/srd-5.2.1/magic_item/ammunition_1_2_or_3.md
  pages:
  - 209
  engine_gaps: &id001
  - dnd2024:item_ammunition_1_2_or_3
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: cf6cf3e9d880a6ea4ef8ce5af7a5a5907bc1d410be3a9016e7f7d02896d00c6f
splat_clauses:
- key: source_rules
  text: '# Ammunition, +1, +2, or +3 Weapon (Any Ammunition), Uncommon (+1), Rare (+2), or Very Rare (+3)
    You have a bonus to attack rolls and damage rolls made with this piece of magic ammunition. The bonus
    is determined by the rarity of the ammunition. Once it hits a target, the ammunition is no longer
    magical. This ammunition is typically found or sold in quantities of ten or twenty pieces. Ten pieces
    of this ammunition are equivalent in value to a potion of the same rarity.'
  sha256: 180f8c58f866b6fe3605591ce32424c98b9fe649e0b765fcff3f21159824eda5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ammunition, +1, +2, or +3

<!-- SRD page 209 -->

Weapon (Any Ammunition), Uncommon (+1), Rare (+2),\
or Very Rare (+3)\
You have a bonus to attack rolls and damage rolls\
made with this piece of magic ammunition. The bo-\
nus is determined by the rarity of the ammunition.\
Once it hits a target, the ammunition is no longer\
magical.\
This ammunition is typically found or sold in\
quantities of ten or twenty pieces. Ten pieces of this\
ammunition are equivalent in value to a potion of\
the same rarity.\

Original layout: [[../../srd-5.2.1/pages/209|page 209]].
