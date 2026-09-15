---
splat_schema: 1
splat_id: 1b29595a-8b0a-54a2-ac5a-12fe422c33e6
splat_kind: item
splat_key: dnd2024:items/shield_1_2_or_3
splat_name: Shield, +1, +2, or +3
splat_origin:
  key: dnd2024:reference/magic_item/shield_1_2_or_3
splat_sources:
- title: SRD 5.2.1
  locator: pp. 243
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/shield_1_2_or_3
  implementation: dnd2024:item_shield_1_2_or_3
  parameters:
    equipment_state: held
    variants:
    - rarity: uncommon
      ac_bonus: 1
      additional_to_normal_shield_bonus: true
    - rarity: rare
      ac_bonus: 2
      additional_to_normal_shield_bonus: true
    - rarity: very_rare
      ac_bonus: 3
      additional_to_normal_shield_bonus: true
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_armor
splat_contracts:
- id: dnd2024:item_shield_1_2_or_3
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
  - equipment.changed
  capabilities:
  - inventory
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 99a855a25aef7e21b6d52c16bbc6b28fe35857593459eb63c7a3c3e2cf83334a
  source_path: builtin/srd-5.2.1/magic_item/shield_1_2_or_3.md
  pages:
  - 243
  engine_gaps: &id001
  - dnd2024:item_shield_1_2_or_3
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 4cb653df727ec054eef90df686d35435bd1af00fc6b30e645ca7c80391ca0d1b
splat_clauses:
- key: source_rules
  text: '# Shield, +1, +2, or +3 Armor (Shield), Uncommon (+1), Rare (+2), or Very Rare (+3) While holding
    this Shield, you have a bonus to Armor Class determined by the Shield’s rarity, in addition to the
    Shield’s normal bonus to AC.'
  sha256: bddc31f270347f70cc11ba2039dd02c10d83351351357357db3d8ddfee9bbcad
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shield, +1, +2, or +3

<!-- SRD page 243 -->

Armor (Shield), Uncommon (+1), Rare (+2), or Very\
Rare (+3)\
While holding this Shield, you have a bonus to Ar-\
mor Class determined by the Shield’s rarity, in addi-\
tion to the Shield’s normal bonus to AC.\

Original layout: [[../../srd-5.2.1/pages/243|page 243]].
