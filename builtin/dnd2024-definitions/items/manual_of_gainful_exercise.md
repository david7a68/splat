---
splat_schema: 1
splat_id: be8a502a-255c-5496-9d4b-2175967d22b3
splat_kind: item
splat_key: dnd2024:items/manual_of_gainful_exercise
splat_name: Manual of Gainful Exercise
splat_origin:
  key: dnd2024:reference/magic_item/manual_of_gainful_exercise
splat_sources:
- title: SRD 5.2.1
  locator: pp. 229
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/manual_of_gainful_exercise
  implementation: dnd2024:item_manual_of_gainful_exercise
  parameters:
    study:
      hours: 48
      within_days: 6
      practice_required: true
    completion:
      ability: strength
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
- id: dnd2024:item_manual_of_gainful_exercise
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
  source_sha256: 7bae9cf795e49824a3d70822c0f72cebf6a2954a69199406ea2f22e487b56fcc
  source_path: builtin/srd-5.2.1/magic_item/manual_of_gainful_exercise.md
  pages:
  - 229
  engine_gaps: &id001
  - dnd2024:item_manual_of_gainful_exercise
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 649af57ac8bca3ab37d98b88e30021239ce2a245c94cad427bd6287ab7212641
splat_clauses:
- key: source_rules
  text: '# Manual of Gainful Exercise Wondrous Item, Very Rare This book describes fitness exercises,
    and its words are charged with magic. If you spend 48 hours over a period of 6 days or fewer studying
    the book’s contents and practicing its guidelines, your Strength increases by 2, to a maximum of 30.
    The manual then loses its magic but regains it in a century.'
  sha256: 71668bb342847a13c495a0567274be0a47a07b37a9878de7b5df48ed7deb8bd5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Manual of Gainful Exercise

<!-- SRD page 229 -->

Wondrous Item, Very Rare\
This book describes fitness exercises, and its words\
are charged with magic. If you spend 48 hours over\
a period of 6 days or fewer studying the book’s con-\
tents and practicing its guidelines, your Strength in-\
creases by 2, to a maximum of 30. The manual then\
loses its magic but regains it in a century.\

Original layout: [[../../srd-5.2.1/pages/229|page 229]].
