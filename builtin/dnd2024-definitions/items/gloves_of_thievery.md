---
splat_schema: 1
splat_id: d5a04c8e-8259-516d-9ba6-0bc5d8fe607a
splat_kind: item
splat_key: dnd2024:items/gloves_of_thievery
splat_name: Gloves of Thievery
splat_origin:
  key: dnd2024:reference/magic_item/gloves_of_thievery
splat_sources:
- title: SRD 5.2.1
  locator: pp. 224
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/gloves_of_thievery
  implementation: dnd2024:item_gloves_of_thievery
  parameters:
    equipped: worn
    item_imperceptible: true
    checks:
    - ability: dexterity
      skill: sleight_of_hand
      bonus: 5
    attunement_required: false
splat_contracts:
- id: dnd2024:item_gloves_of_thievery
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
  - ability_check.pending
  capabilities:
  - item_effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: a571ccdb68b9df7a421e67a7edae4248d2b628b855ebb27871b02e7d73d3cc21
  source_path: builtin/srd-5.2.1/magic_item/gloves_of_thievery.md
  pages:
  - 224
  engine_gaps: &id001
  - dnd2024:item_gloves_of_thievery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 447550284dddd503a11b8acc0fe6cdb9cc3beebed41fd4991b9ea25838904920
splat_clauses:
- key: source_rules
  text: '# Gloves of Thievery Wondrous Item, Uncommon These gloves are imperceptible while worn. While
    wearing them, you gain a +5 bonus to Dexterity (Sleight of Hand) checks.'
  sha256: 065867aee754ca37f71757849f984103879195f4b8826935f79158a6336e7ae0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gloves of Thievery

<!-- SRD page 224 -->

Wondrous Item, Uncommon\
These gloves are imperceptible while worn. While\
wearing them, you gain a +5 bonus to Dexterity\
(Sleight of Hand) checks.\

Original layout: [[../../srd-5.2.1/pages/224|page 224]].
