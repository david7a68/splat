---
splat_schema: 1
splat_id: cb7096af-bc9c-59a0-a8de-ec1b26e70cc4
splat_kind: item
splat_key: dnd2024:items/ring_of_protection
splat_name: Ring of Protection
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_protection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_protection
  implementation: dnd2024:item_ring_of_protection
  parameters:
    equipment_state: worn_on_finger_or_similar_digit
    ac_bonus: 1
    saving_throw_bonus: 1
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_rings
splat_contracts:
- id: dnd2024:item_ring_of_protection
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
  source_sha256: 1c7e9e4841fe437e774945d51766e16bd3c4f5091c6854b050cab1d59c1263ef
  source_path: builtin/srd-5.2.1/magic_item/ring_of_protection.md
  pages:
  - 238
  engine_gaps: &id001
  - dnd2024:item_ring_of_protection
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 280cf6768bb69383530e738d9db2bd8ad1b15d2371fb07a3b73e39dbf56e61bd
splat_clauses:
- key: source_rules
  text: '# Ring of Protection Ring, Rare (Requires Attunement) You gain a +1 bonus to Armor Class and
    saving throws while wearing this ring.'
  sha256: f8453f1d8bb49126e93179b6f753bbb4c7d5e5299dce3d175249d8cea4c7baf8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Protection

<!-- SRD page 238 -->

Ring, Rare (Requires Attunement)\
You gain a +1 bonus to Armor Class and saving\
throws while wearing this ring.\

Original layout: [[../../srd-5.2.1/pages/238|page 238]].
