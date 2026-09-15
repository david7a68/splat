---
splat_schema: 1
splat_id: 2890aabd-bede-5824-a607-9e09cb81461c
splat_kind: item
splat_key: dnd2024:items/pearl_of_power
splat_name: Pearl of Power
splat_origin:
  key: dnd2024:reference/magic_item/pearl_of_power
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
    attunement_requirement_text: a Spellcaster
splat_rule:
  mechanic: dnd2024:items/pearl_of_power
  implementation: dnd2024:item_pearl_of_power
  parameters:
    equipped: on_person
    attunement_requires: spellcaster
    action: magic
    restore_slot:
      count: 1
      expended_only: true
      level_min: 1
      level_max: 3
      choice: user
    uses:
      maximum: 1
      recovery_event: time.dawn
    attunement_required: true
splat_contracts:
- id: dnd2024:item_pearl_of_power
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
  - item.activation_requested
  - time.dawn
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 53fb70b881bef6a18ae8bc316999d2235c5da21d9aae8d03d7216575a1ce16e2
  source_path: builtin/srd-5.2.1/magic_item/pearl_of_power.md
  pages:
  - 234
  engine_gaps: &id001
  - dnd2024:item_pearl_of_power
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 72079da5ffe6ff957d466062182b9ee3e60df48a6c9e96c9b65c534079f62413
splat_clauses:
- key: source_rules
  text: '# Pearl of Power Wondrous Item, Uncommon (Requires Attunement by a Spellcaster) While this pearl
    is on your person, you can take a Magic action to regain one expended spell slot of level 3 or lower.
    Once you use the pearl, it can’t be used again until the next dawn.'
  sha256: 5ade154f02a5516f62092d7b5b25ecfca5108b2b51610c6891173ef5bce93bca
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pearl of Power

<!-- SRD page 234 -->

Wondrous Item, Uncommon (Requires Attunement by\
a Spellcaster)\
While this pearl is on your person, you can take a\
Magic action to regain one expended spell slot of\
level 3 or lower. Once you use the pearl, it can’t be\
used again until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]].
