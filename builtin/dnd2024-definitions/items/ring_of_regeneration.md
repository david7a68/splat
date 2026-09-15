---
splat_schema: 1
splat_id: 49f81995-bc49-58d8-b972-06c8e6b43f24
splat_kind: item
splat_key: dnd2024:items/ring_of_regeneration
splat_name: Ring of Regeneration
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_regeneration
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_regeneration
  implementation: dnd2024:item_ring_of_regeneration
  parameters:
    equipped: worn
    healing:
      amount:
        roll:
          count: 1
          sides: 6
      interval:
        amount: 10
        unit: minutes
      requires_hp_minimum: 1
    regrow_part:
      trigger: body_part_lost
      duration:
        amount:
          roll:
            count: 1
            sides: 6
            add: 1
        unit: days
      requires_hp_minimum_throughout: 1
      full_functionality: true
    attunement_required: true
splat_contracts:
- id: dnd2024:item_ring_of_regeneration
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
  - time.elapsed
  - body_part.lost
  - hp.changed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - effect_duration
  - healing
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: c00ccd3f1351c3119ba551f7c3280e9e4e32975986daf091da383a9c7d7a824c
  source_path: builtin/srd-5.2.1/magic_item/ring_of_regeneration.md
  pages:
  - 238
  engine_gaps: &id001
  - dnd2024:item_ring_of_regeneration
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: d6328438a51950af896fd6d6ecd9f7e813c0af3cfb67781b01c5226d3a4608b8
splat_clauses:
- key: source_rules
  text: '# Ring of Regeneration Ring, Very Rare (Requires Attunement) While wearing this ring, you regain
    1d6 Hit Points every 10 minutes if you have at least 1 Hit Point. If you lose a body part, the ring
    causes the missing part to regrow and return to full functionality after 1d6 + 1 days if you have
    at least 1 Hit Point the whole time.'
  sha256: aed0401c1fa11300175f64724e08db61f9a59f9801b463e5e69c745f5ab2c74a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Regeneration

<!-- SRD page 238 -->

Ring, Very Rare (Requires Attunement)\
While wearing this ring, you regain 1d6 Hit Points\
every 10 minutes if you have at least 1 Hit Point. If\
you lose a body part, the ring causes the missing\
part to regrow and return to full functionality after\
1d6 + 1 days if you have at least 1 Hit Point the\
whole time.\

Original layout: [[../../srd-5.2.1/pages/238|page 238]].
