---
splat_schema: 1
splat_id: adaee6e3-5a29-5e3d-8d5b-3de170c7b8c9
splat_kind: item
splat_key: dnd2024:items/ring_of_jumping
splat_name: Ring of Jumping
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_jumping
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_jumping
  implementation: dnd2024:item_ring_of_jumping
  parameters:
    equipped: worn
    spells:
    - spell:
        key: dnd2024:spells/jump
      target: self
      usage: at_will
    attunement_required: true
splat_contracts:
- id: dnd2024:item_ring_of_jumping
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
  source_sha256: 9dc9190c2f1e9929c726a65c9048804ad10f8a1eb4a09dafae8dc67e347b0c13
  source_path: builtin/srd-5.2.1/magic_item/ring_of_jumping.md
  pages:
  - 238
  engine_gaps: &id001
  - dnd2024:item_ring_of_jumping
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 2260bd369a6c1528254290bd33a085009281813b142f2b96a78b59c245776507
splat_clauses:
- key: source_rules
  text: '# Ring of Jumping Ring, Uncommon (Requires Attunement) While wearing this ring, you can cast
    Jump from it, but can target only yourself when you do so.'
  sha256: 2f394db4c180def6acdc36d7118ba372c41ce139e00b12909fdcce724d19a9e2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Jumping

<!-- SRD page 238 -->

Ring, Uncommon (Requires Attunement)\
While wearing this ring, you can cast Jump from it,\
but can target only yourself when you do so.\

Original layout: [[../../srd-5.2.1/pages/238|page 238]].
