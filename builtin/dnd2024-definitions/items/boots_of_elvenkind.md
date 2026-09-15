---
splat_schema: 1
splat_id: fa387b75-f756-59fe-81e1-37fa1384c3b6
splat_kind: item
splat_key: dnd2024:items/boots_of_elvenkind
splat_name: Boots of Elvenkind
splat_origin:
  key: dnd2024:reference/magic_item/boots_of_elvenkind
splat_sources:
- title: SRD 5.2.1
  locator: pp. 213
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/boots_of_elvenkind
  implementation: dnd2024:item_boots_of_elvenkind
  parameters:
    equipment_state: worn
    steps_silent:
      all_surfaces: true
    check_advantage:
    - ability: dexterity
      skill: stealth
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_boots_of_elvenkind
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
  - movement.pending
  - check.pending
  capabilities:
  - inventory
  - effects
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 8167ffbc88c31c34c149c57916d12aca8f7349153b71551ad03c98c842775ac8
  source_path: builtin/srd-5.2.1/magic_item/boots_of_elvenkind.md
  pages:
  - 213
  engine_gaps: &id001
  - dnd2024:item_boots_of_elvenkind
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: b35dca3d958d1a59288bec137c001423ac03ad6d2f62981c33e9db3f794068b6
splat_clauses:
- key: source_rules
  text: '# Boots of Elvenkind Wondrous Item, Uncommon While you wear these boots, your steps make no sound,
    regardless of the surface you are moving across. You also have Advantage on Dexterity (Stealth) checks.'
  sha256: a3cb9b005b16af29ae5646e72e530e00a1988bad42a97f63a1c33fb229cc0031
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boots of Elvenkind

<!-- SRD page 213 -->

Wondrous Item, Uncommon\
While you wear these boots, your steps make no\
sound, regardless of the surface you are moving\
across. You also have Advantage on Dexterity\
(Stealth) checks.\

Original layout: [[../../srd-5.2.1/pages/213|page 213]].
