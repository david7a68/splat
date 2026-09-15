---
splat_schema: 1
splat_id: 4dbe2137-d0a6-5ecd-9dce-fc02548f2a1c
splat_kind: item
splat_key: dnd2024:items/circlet_of_blasting
splat_name: Circlet of Blasting
splat_origin:
  key: dnd2024:reference/magic_item/circlet_of_blasting
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/circlet_of_blasting
  implementation: dnd2024:item_circlet_of_blasting
  parameters:
    equipped: worn
    spells:
    - spell:
        key: dnd2024:spells/scorching_ray
      attack_bonus: 5
      uses:
        maximum: 1
        recovery_event: time.dawn
    attunement_required: false
splat_contracts:
- id: dnd2024:item_circlet_of_blasting
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
  capabilities:
  - item_effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: b1c3e23d49a844543ac61008d2c4398ed6575d82a039d65284e8260d3b859131
  source_path: builtin/srd-5.2.1/magic_item/circlet_of_blasting.md
  pages:
  - 215
  engine_gaps: &id001
  - dnd2024:item_circlet_of_blasting
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: ccb91dff0bf6867c5882ae481745cd83410139aff934d85b40761f9a1a233561
splat_clauses:
- key: source_rules
  text: '# Circlet of Blasting Wondrous Item, Uncommon While wearing this circlet, you can cast Scorching
    Ray with it (+5 to hit). The circlet can’t cast this spell again until the next dawn.'
  sha256: cdce471e156a5b5a89da9f4afee758e25deb83fb93b7dec05d20cc7184a115a3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Circlet of Blasting

<!-- SRD page 215 -->

Wondrous Item, Uncommon\
While wearing this circlet, you can cast Scorching\
Ray with it (+5 to hit). The circlet can’t cast this spell\
again until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/215|page 215]].
