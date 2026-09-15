---
splat_schema: 1
splat_id: 852f4cfa-561d-5617-ac29-10e26082efe0
splat_kind: item
splat_key: dnd2024:items/gloves_of_swimming_and_climbing
splat_name: Gloves of Swimming and Climbing
splat_origin:
  key: dnd2024:reference/magic_item/gloves_of_swimming_and_climbing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 224
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/gloves_of_swimming_and_climbing
  implementation: dnd2024:item_gloves_of_swimming_and_climbing
  parameters:
    equipment_state: worn
    climb_speed: speed
    swim_speed: speed
    check_bonus:
      ability: strength
      skill: athletics
      tasks:
      - climb
      - swim
      amount: 5
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_gloves_of_swimming_and_climbing
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
  - equipment.changed
  - speed.changed
  - check.pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 564dc71a5a98993637094c6d3f86ee0efa608ccdec6a54b99db059ddfe4421b6
  source_path: builtin/srd-5.2.1/magic_item/gloves_of_swimming_and_climbing.md
  pages:
  - 224
  engine_gaps: &id001
  - dnd2024:item_gloves_of_swimming_and_climbing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 4d85254e5c4a50560ae85bedc14727da37d170223841a8e048511444164e01ff
splat_clauses:
- key: source_rules
  text: '# Gloves of Swimming and Climbing Wondrous Item, Uncommon (Requires Attunement) While wearing
    these gloves, you have a Climb Speed and a Swim Speed equal to your Speed, and you gain a +5 bonus
    to Strength (Athletics) checks made to climb or swim.'
  sha256: 93014513e5283736e93706ab7ebb2ca452248c79d4a60731cb81a02e2a6fd23b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gloves of Swimming and Climbing

<!-- SRD page 224 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing these gloves, you have a Climb Speed\
and a Swim Speed equal to your Speed, and you gain\
a +5 bonus to Strength (Athletics) checks made to\
climb or swim.\

Original layout: [[../../srd-5.2.1/pages/224|page 224]].
