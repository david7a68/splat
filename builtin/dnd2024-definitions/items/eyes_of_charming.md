---
splat_schema: 1
splat_id: d3117ef7-c32c-50e2-a05e-f718d87a83ca
splat_kind: item
splat_key: dnd2024:items/eyes_of_charming
splat_name: Eyes of Charming
splat_origin:
  key: dnd2024:reference/magic_item/eyes_of_charming
splat_sources:
- title: SRD 5.2.1
  locator: pp. 221
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/eyes_of_charming
  implementation: dnd2024:item_eyes_of_charming
  parameters:
    equipped: worn
    charges:
      initial: 3
      maximum: 3
      recovery:
        event: time.dawn
        amount: all
        cap_at_maximum: true
    spells:
    - spell:
        key: dnd2024:spells/charm_person
      save_dc: 13
      charge_choice:
        min: 1
        max: available_charges
      cast_level: charges_spent
    attunement_required: true
splat_contracts:
- id: dnd2024:item_eyes_of_charming
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
  - time.dawn
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - item_spellcasting
  - item_charges
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 0d5f5e2c411a93a2070161983bea61e4fc6fd91801769e06cff69bf903760b8c
  source_path: builtin/srd-5.2.1/magic_item/eyes_of_charming.md
  pages:
  - 221
  engine_gaps: &id001
  - dnd2024:item_eyes_of_charming
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 1ea27a951ac5a395b22c4d16895fa9f8cfea7c0701e2b598ce5ad8393d7f336a
splat_clauses:
- key: source_rules
  text: '# Eyes of Charming Wondrous Item, Uncommon (Requires Attunement) These crystal lenses fit over
    the eyes. They have 3 charges. While wearing them, you can expend 1 or more charges to cast Charm
    Person (save DC 13). For 1 charge, you cast the level 1 version of the spell. You increase the spell’s
    level by one for each additional charge you expend. The lenses regain all expended charges daily at
    dawn.'
  sha256: 9b41f9690b84375472658e06b7b5af0019a986ea281d884ecce1a70ac90d25c6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Eyes of Charming

<!-- SRD page 221 -->

Wondrous Item, Uncommon (Requires Attunement)\
These crystal lenses fit over the eyes. They have\
3 charges. While wearing them, you can expend\
1 or more charges to cast Charm Person (save DC\
13). For 1 charge, you cast the level 1 version of the\
spell. You increase the spell’s level by one for each\
additional charge you expend. The lenses regain all\
expended charges daily at dawn.\

Original layout: [[../../srd-5.2.1/pages/221|page 221]].
