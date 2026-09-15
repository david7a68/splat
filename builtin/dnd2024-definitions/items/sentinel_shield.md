---
splat_schema: 1
splat_id: 842752d3-d1b1-51bc-a8dc-9fb075f0617a
splat_kind: item
splat_key: dnd2024:items/sentinel_shield
splat_name: Sentinel Shield
splat_origin:
  key: dnd2024:reference/magic_item/sentinel_shield
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
  mechanic: dnd2024:items/sentinel_shield
  implementation: dnd2024:item_sentinel_shield
  parameters:
    equipment_state: held
    advantage:
    - initiative
    - wisdom_perception
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_sentinel_shield
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
  source_sha256: 4601befb7370a2b875048575fa9f0ba6f0bd02f63f3646ca1793a585a7ce02c9
  source_path: builtin/srd-5.2.1/magic_item/sentinel_shield.md
  pages:
  - 243
  engine_gaps: &id001
  - dnd2024:item_sentinel_shield
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: b7285783bc2a4a4b351fe7f611387a46f6bb8731b3a54f40aa9a9023b374d62d
splat_clauses:
- key: source_rules
  text: '# Sentinel Shield Armor (Shield), Uncommon While holding this Shield, you have Advantage on Initiative
    rolls and Wisdom (Perception) checks. The Shield is emblazoned with a symbol of an eye.'
  sha256: e6b12be5d7cf00a31917679bf3f6d413e78d73e08072121975b9f41289e52f6c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sentinel Shield

<!-- SRD page 243 -->

Armor (Shield), Uncommon\
While holding this Shield, you have Advantage on\
Initiative rolls and Wisdom (Perception) checks.\
The Shield is emblazoned with a symbol of an eye.\

Original layout: [[../../srd-5.2.1/pages/243|page 243]].
