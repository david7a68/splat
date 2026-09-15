---
splat_schema: 1
splat_id: 8867b69e-fad0-5895-9ac4-1a490d43be74
splat_kind: item
splat_key: dnd2024:items/carpet_of_flying
splat_name: Carpet of Flying
splat_origin:
  key: dnd2024:reference/magic_item/carpet_of_flying
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/carpet_of_flying
  implementation: dnd2024:item_carpet_of_flying
  parameters:
    activate:
      action: magic
      command_word: true
      mode: hovering_flying
    direction_control_range_ft: 30
    variant_selection: gm_choice_or_roll
    variants:
    - roll:
      - 1
      - 20
      dimensions_ft:
      - 3
      - 5
      normal_capacity_lb: 200
      fly_speed_ft: 80
    - roll:
      - 21
      - 55
      dimensions_ft:
      - 4
      - 6
      normal_capacity_lb: 400
      fly_speed_ft: 60
    - roll:
      - 56
      - 80
      dimensions_ft:
      - 5
      - 7
      normal_capacity_lb: 600
      fly_speed_ft: 40
    - roll:
      - 81
      - 100
      dimensions_ft:
      - 6
      - 9
      normal_capacity_lb: 800
      fly_speed_ft: 30
    variant_die: 100
    maximum_capacity_multiplier: 2
    over_normal_capacity_speed_multiplier: 0.5
    attunement_required: false
splat_contracts:
- id: dnd2024:item_carpet_of_flying
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
  - item.load_changed
  capabilities:
  - item_effects
  - dice
  - spatial_targeting
  - item_variants
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: febbafb35f07204f3340f51f9e969a80c6de588135b52a954c42734fb0a8a88f
  source_path: builtin/srd-5.2.1/magic_item/carpet_of_flying.md
  pages:
  - 215
  engine_gaps: &id001
  - dnd2024:item_carpet_of_flying
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 599cbed81cff75a3d5a5a756a22cbb64f679c5e4e00b8ce6066cfbcfd8fa2a0d
splat_clauses:
- key: source_rules
  text: '# Carpet of Flying Wondrous Item, Very Rare You can make this carpet hover and fly by taking
    a Magic action and using the carpet’s command word. It moves according to your directions if you are
    within 30 feet of it. Four sizes of Carpet of Flying exist. The GM chooses the size of a given carpet
    or determines it randomly by rolling on the following table. A carpet can carry up to twice the weight
    shown on the table, but its Fly Speed is halved if it carries more than its normal capacity. 1d100
    Size Capacity Fly Speed 01–20 3 ft. × 5 ft. 200 lb. 80 feet 21–55 4 ft. × 6 ft. 400 lb. 60 feet 56–80
    5 ft. × 7 ft. 600 lb. 40 feet 81–00 6 ft. × 9 ft. 800 lb. 30 feet'
  sha256: 21446021dd62c71ce150c63faa87fc73e40fe367a4c0004b609d5d62e15725c0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Carpet of Flying

<!-- SRD page 215 -->

Wondrous Item, Very Rare\
You can make this carpet hover and fly by taking a\
Magic action and using the carpet’s command word.\
It moves according to your directions if you are\
within 30 feet of it.\
Four sizes of Carpet of Flying exist. The GM\
chooses the size of a given carpet or determines it\
randomly by rolling on the following table. A carpet\
can carry up to twice the weight shown on the table,\
but its Fly Speed is halved if it carries more than its\
normal capacity.\
1d100 Size Capacity Fly Speed\
01–20 3 ft. × 5 ft. 200 lb. 80 feet\
21–55 4 ft. × 6 ft. 400 lb. 60 feet\
56–80 5 ft. × 7 ft. 600 lb. 40 feet\
81–00 6 ft. × 9 ft. 800 lb. 30 feet\

Original layout: [[../../srd-5.2.1/pages/215|page 215]].
