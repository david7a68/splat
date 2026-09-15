---
splat_schema: 1
splat_id: ad64e00b-cd76-5a1e-86d8-83a29ebc8a6a
splat_kind: item
splat_key: dnd2024:items/armor_1_2_or_3
splat_name: Armor, +1, +2, or +3
splat_origin:
  key: dnd2024:reference/magic_item/armor_1_2_or_3
splat_sources:
- title: SRD 5.2.1
  locator: pp. 210
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/armor_1_2_or_3
  implementation: dnd2024:item_armor_1_2_or_3
  parameters:
    equipped: worn
    variants:
    - rarity: rare
      ac_bonus: 1
    - rarity: very_rare
      ac_bonus: 2
    - rarity: legendary
      ac_bonus: 3
    base_armor_categories:
    - light
    - medium
    - heavy
    attunement_required: false
splat_contracts:
- id: dnd2024:item_armor_1_2_or_3
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - item.equipment_changed
  capabilities:
  - item_effects
  - item_variants
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: b7d0b03093807c512288a0bc54a8266846fd85aa9410254d55c40537175b8730
  source_path: builtin/srd-5.2.1/magic_item/armor_1_2_or_3.md
  pages:
  - 210
  engine_gaps: &id001
  - dnd2024:item_armor_1_2_or_3
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 4e2ff9967559c161f2acae09fb329d79424d9a2efde5d762c40b73fec784f59b
splat_clauses:
- key: source_rules
  text: '# Armor, +1, +2, or +3 Armor (Any Light, Medium, or Heavy), Rare (+1), Very Rare (+2), or Legendary
    (+3) You have a bonus to Armor Class while wearing this armor. The bonus is determined by its rarity.'
  sha256: 4734d1769722b458dda76e1df1294908521e1d8b8ee7b74e82791c228a469643
  status: contract_specified
splat_requires_mechanics: *id001
---

# Armor, +1, +2, or +3

<!-- SRD page 210 -->

Armor (Any Light, Medium, or Heavy), Rare (+1), Very\
Rare (+2), or Legendary (+3)\
You have a bonus to Armor Class while wearing this\
armor. The bonus is determined by its rarity.\

Original layout: [[../../srd-5.2.1/pages/210|page 210]].
