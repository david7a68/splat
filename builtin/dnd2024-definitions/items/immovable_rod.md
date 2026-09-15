---
splat_schema: 1
splat_id: b72b7339-8954-5e4e-acee-5e8da4331a86
splat_kind: item
splat_key: dnd2024:items/immovable_rod
splat_name: Immovable Rod
splat_origin:
  key: dnd2024:reference/magic_item/immovable_rod
splat_sources:
- title: SRD 5.2.1
  locator: pp. 227
  url: https://www.dndbeyond.com/srd
splat_item:
  category: rod
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/immovable_rod
  implementation: dnd2024:item_immovable_rod
  parameters:
    toggle:
      action: utilize
      actor: any_creature
      operation: press_button
      states:
      - fixed
      - unfixed
    fixed:
      gravity_ignored: true
      maximum_load_lb: 8000
      overload:
      - deactivate
      - fall
      move:
        action: utilize
        check:
          ability: strength
          skill: athletics
          dc: 30
        success_distance_max_ft: 10
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_immovable_rod
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
  - spatial_queries
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 8c9673110615e8473d55eeae8d08dd15b980868afe65739960b84f05ddd9d2d0
  source_path: builtin/srd-5.2.1/magic_item/immovable_rod.md
  pages:
  - 227
  engine_gaps: &id001
  - dnd2024:item_immovable_rod
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: d71be3277082d249b717457bd3f5a094607cb199fc8e9805aa3c74c385b48658
splat_clauses:
- key: source_rules
  text: '# Immovable Rod Rod, Uncommon This iron rod has a button on one end. You can take a Utilize action
    to press the button, which causes the rod to become magically fixed in place. Until you or another
    creature takes a Utilize action to push the button again, the rod doesn’t move, even if it defies
    gravity. The rod can hold up to 8,000 pounds of weight. More weight causes the rod to deactivate and
    fall. A creature can take a Utilize action to make a DC 30 Strength (Athletics) check, moving the
    fixed rod up to 10 feet on a successful check.'
  sha256: 50e6fc725acb7d02a1adb2a3b637afe324bef28e425941e0a78a0391d8f38444
  status: contract_specified
splat_requires_mechanics: *id001
---

# Immovable Rod

<!-- SRD page 227 -->

Rod, Uncommon\
This iron rod has a button on one end. You can take\
a Utilize action to press the button, which causes\
the rod to become magically fixed in place. Until you\
or another creature takes a Utilize action to push\
the button again, the rod doesn’t move, even if it\
defies gravity. The rod can hold up to 8,000 pounds\
of weight. More weight causes the rod to deactivate\
and fall. A creature can take a Utilize action to make\
a DC 30 Strength (Athletics) check, moving the fixed\
rod up to 10 feet on a successful check.\

Original layout: [[../../srd-5.2.1/pages/227|page 227]].
