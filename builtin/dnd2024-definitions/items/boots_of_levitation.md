---
splat_schema: 1
splat_id: a462de3b-b0f7-5b2d-9fb3-107759b2ea62
splat_kind: item
splat_key: dnd2024:items/boots_of_levitation
splat_name: Boots of Levitation
splat_origin:
  key: dnd2024:reference/magic_item/boots_of_levitation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 213
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/boots_of_levitation
  implementation: dnd2024:item_boots_of_levitation
  parameters:
    equipped: worn
    spells:
    - spell:
        key: dnd2024:spells/levitate
      target: self
      usage: at_will
    attunement_required: true
splat_contracts:
- id: dnd2024:item_boots_of_levitation
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
  - spell.cast_pending
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: c433d593a4095c700e086bf41ae61054f9e533219fee606020c8406309f9ef6e
  source_path: builtin/srd-5.2.1/magic_item/boots_of_levitation.md
  pages:
  - 213
  engine_gaps: &id001
  - dnd2024:item_boots_of_levitation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 5048714c61c819659f4ff78c198e3d03e6d4af55c9f52e519a58d444c8e9a29d
splat_clauses:
- key: source_rules
  text: '# Boots of Levitation Wondrous Item, Rare (Requires Attunement) While you wear these boots, you
    can cast Levitate on yourself.'
  sha256: 61e29be65b611865c2a5f633d7d7e723b2066f8a8423b853cd70a87b55513eae
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boots of Levitation

<!-- SRD page 213 -->

Wondrous Item, Rare (Requires Attunement)\
While you wear these boots, you can cast Levitate\
on yourself.\

Original layout: [[../../srd-5.2.1/pages/213|page 213]].
