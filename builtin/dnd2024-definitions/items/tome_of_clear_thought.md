---
splat_schema: 1
splat_id: c205b4a5-f3dc-527a-b7d7-38b0ee8afbd8
splat_kind: item
splat_key: dnd2024:items/tome_of_clear_thought
splat_name: Tome of Clear Thought
splat_origin:
  key: dnd2024:reference/magic_item/tome_of_clear_thought
splat_sources:
- title: SRD 5.2.1
  locator: pp. 249
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/tome_of_clear_thought
  implementation: dnd2024:item_tome_of_clear_thought
  parameters:
    study:
      hours: 48
      within_days: 6
      practice_required: true
    completion:
      ability: intelligence
      increase: 2
      maximum: 30
      duration: permanent
      item_magical: false
    recovery:
      after:
        amount: 100
        unit: year
      item_magical: true
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_tome_of_clear_thought
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
  - study.completed
  - time.elapsed
  capabilities:
  - inventory
  - effects
  - resources_and_time
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: f243d1f1f4335693413058980eee1d999623e6d91efcf29e891d61b8fb895d20
  source_path: builtin/srd-5.2.1/magic_item/tome_of_clear_thought.md
  pages:
  - 249
  engine_gaps: &id001
  - dnd2024:item_tome_of_clear_thought
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 2d8ce2e34e96f9af67ac9c9faa85e243e39239f093984ae3cf4d97793ee4adba
splat_clauses:
- key: source_rules
  text: '# Tome of Clear Thought Wondrous Item, Very Rare This book contains memory and logic exercises,
    and its words are charged with magic. If you spend 48 hours over a period of 6 days or fewer studying
    the book’s contents and practicing its guidelines, your Intelligence increases by 2, to a maximum
    of 30. The manual then loses its magic but regains it in a century.'
  sha256: 0272984f7b60dcb581392daa9d5338d6135abf687f01772a4be14effab5d68af
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tome of Clear Thought

<!-- SRD page 249 -->

Wondrous Item, Very Rare\
This book contains memory and logic exercises,\
and its words are charged with magic. If you spend\
48 hours over a period of 6 days or fewer studying\
the book’s contents and practicing its guidelines,\
your Intelligence increases by 2, to a maximum of\
30. The manual then loses its magic but regains it in\
a century.\

Original layout: [[../../srd-5.2.1/pages/249|page 249]].
