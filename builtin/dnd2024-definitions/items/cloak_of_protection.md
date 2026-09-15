---
splat_schema: 1
splat_id: d81e65a5-6499-5575-ab01-276809f5c7a0
splat_kind: item
splat_key: dnd2024:items/cloak_of_protection
splat_name: Cloak of Protection
splat_origin:
  key: dnd2024:reference/magic_item/cloak_of_protection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/cloak_of_protection
  implementation: dnd2024:item_cloak_of_protection
  parameters:
    equipment_state: worn
    ac_bonus: 1
    saving_throw_bonus: 1
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_cloak_of_protection
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
  - saving_throw.pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 2911956a4eace33b125fb7650ca98458c8544ab1fa81080e2dd00129484943cb
  source_path: builtin/srd-5.2.1/magic_item/cloak_of_protection.md
  pages:
  - 216
  engine_gaps: &id001
  - dnd2024:item_cloak_of_protection
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 2d7d034a27b15fae5cb163cc121da6006b35a5453a988d3a3c8027b6943a36e3
splat_clauses:
- key: source_rules
  text: '# Cloak of Protection Wondrous Item, Uncommon (Requires Attunement) You gain a +1 bonus to Armor
    Class and saving throws while you wear this cloak.'
  sha256: 89114b6ad143b5ac47d1dc87cf5dab71d9baa13038e6ab9883af9f3ef6f9225d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cloak of Protection

<!-- SRD page 216 -->

Wondrous Item, Uncommon (Requires Attunement)\
You gain a +1 bonus to Armor Class and saving\
throws while you wear this cloak.\

Original layout: [[../../srd-5.2.1/pages/216|page 216]].
