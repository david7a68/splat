---
splat_schema: 1
splat_id: e623e0c3-a0b7-5bae-ba4e-818aa4064f97
splat_kind: item
splat_key: dnd2024:items/ring_of_telekinesis
splat_name: Ring of Telekinesis
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_telekinesis
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_telekinesis
  implementation: dnd2024:item_ring_of_telekinesis
  parameters:
    equipped: worn
    spells:
    - spell:
        key: dnd2024:spells/telekinesis
      usage: at_will
    attunement_required: true
splat_contracts:
- id: dnd2024:item_ring_of_telekinesis
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
  source_sha256: d6dd2f42843b7f251a40d174c56c7f411ebb74fb1bb0b625a656feb889a7a1dd
  source_path: builtin/srd-5.2.1/magic_item/ring_of_telekinesis.md
  pages:
  - 239
  engine_gaps: &id001
  - dnd2024:item_ring_of_telekinesis
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 6d2ae146a42b42937e89c730579e4db4556828b0504b032a331bd9223faaec0b
splat_clauses:
- key: source_rules
  text: '# Ring of Telekinesis Ring, Very Rare (Requires Attunement) While wearing this ring, you can
    cast Telekinesis from it.'
  sha256: d67d70ca1fad4d984f66660c48438e2ebbfd406dde30bbe1d3c8765e0aa83803
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Telekinesis

<!-- SRD page 239 -->

Ring, Very Rare (Requires Attunement)\
While wearing this ring, you can cast Telekinesis\
from it.\

Original layout: [[../../srd-5.2.1/pages/239|page 239]].
