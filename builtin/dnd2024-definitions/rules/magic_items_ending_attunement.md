---
splat_schema: 1
splat_id: dfb16e24-c70c-5390-93cd-d4b54d07b50a
splat_kind: rule
splat_key: dnd2024:rules/magic_items_ending_attunement
splat_name: Ending Attunement
splat_origin:
  key: dnd2024:reference/pages/102
splat_sources:
- title: SRD 5.2.1
  locator: pp. 102, 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_items_ending_attunement
  parameters:
    attunement_ends_on:
    - prerequisite_lost
    - bearer_death
    - other_creature_attunes
    separation:
      distance_feet_strictly_greater_than: 100
      continuous_duration:
        value: 24
        unit: hours
    voluntary_end:
      focused_rest: short
      cursed_item_allowed: false
  implementation: dnd2024:rules_magic_items_ending_attunement
splat_contracts:
- id: dnd2024:rules_magic_items_ending_attunement
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      type: choice
      required: true
    event:
      required: false
      type: event
  outputs:
  - item_updates
  - cost
  - effects
  events:
  - attunement.completed
  - rest.short_completed
  - creature.died
  capabilities:
  - items
  - attunement
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 102
  - 103
  engine_gaps: &id001
  - dnd2024:rules_magic_items_ending_attunement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: c030f31fe7a8ec22d62912ca5bd368d5c3af5dbeb76d26fa0eeaad92d3d0920e
  parent_name: Magic Items
  line_span:
    first: 8927
    count: 8
splat_clauses:
- key: source_rules
  text: '# Ending Attunement Your Attunement to an item ends if you no longer satisfy the prerequisites
    for Attunement, if the item has been more than 100 feet away for at least 24 hours, if you die, or
    if another creature attunes to the item. You can also voluntarily end Attunement by spending another
    Short Rest focused on the item unless the item is cursed.'
  sha256: 865b3cce7bc0f9a1628f2541aeb652bbe5c2250b6391c3dea5abbe03c7fb661e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ending Attunement

<!-- SRD page 102 -->

Your Attunement to an item ends if you no longer\
satisfy the prerequisites for Attunement, if the item\
has been more than 100 feet away for at least 24\
hours, if you die, or if another creature attunes to\

<!-- SRD page 103 -->

the item. You can also voluntarily end Attunement\
by spending another Short Rest focused on the item\
unless the item is cursed.\
