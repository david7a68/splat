---
splat_schema: 1
splat_id: adf66340-9c97-5579-95aa-fe5f626a86db
splat_kind: item
splat_key: dnd2024:items/talisman_of_the_sphere
splat_name: Talisman of the Sphere
splat_origin:
  key: dnd2024:reference/magic_item/talisman_of_the_sphere
splat_sources:
- title: SRD 5.2.1
  locator: pp. 249
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/talisman_of_the_sphere
  implementation: dnd2024:item_talisman_of_the_sphere
  parameters:
    equipment_state_any:
    - held
    - worn
    check_advantage:
      ability: intelligence
      skill: arcana
      task: control_sphere_of_annihilation
      item:
        key: dnd2024:items/sphere_of_annihilation
    move_sphere:
      requires: controlling_sphere_at_turn_start
      action: magic
      distance_ft:
        op: add
        args:
        - 10
        - op: multiply
          args:
          - 10
          - stat:
              of: actor
              key: intelligence_modifier
      straight_line_required: false
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_talisman_of_the_sphere
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
  - check.pending
  - turn.started
  - action.magic_pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - roll_resolution
  - spatial_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: a8c0a36ac458cbc4c4004f9824af6bdcc5c88a03d82c347fc3ba2d43c9ed67b4
  source_path: builtin/srd-5.2.1/magic_item/talisman_of_the_sphere.md
  pages:
  - 249
  engine_gaps: &id001
  - dnd2024:item_talisman_of_the_sphere
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 917e78d9493a09c96b963d2c10b5dd59711800f956adb05344bdb30b66c89e17
splat_clauses:
- key: source_rules
  text: '# Talisman of the Sphere Wondrous Item, Legendary (Requires Attunement) While holding or wearing
    this talisman, you have Advantage on any Intelligence (Arcana) check you make to control a Sphere
    of Annihilation. In addition, when you start your turn in control of a Sphere of Annihilation, you
    can take a Magic action to move it 10 feet plus a number of additional feet equal to 10 times your
    Intelligence modifier. This movement doesn’t have to be in a straight line.'
  sha256: 5a5e57ed004b050a4c36c460709d42ddb945f5809e55593cf056191e75ebc8f3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Talisman of the Sphere

<!-- SRD page 249 -->

Wondrous Item, Legendary (Requires Attunement)\
While holding or wearing this talisman, you have\
Advantage on any Intelligence (Arcana) check you\
make to control a Sphere of Annihilation. In addition,\
when you start your turn in control of a Sphere of\
Annihilation, you can take a Magic action to move it\
10 feet plus a number of additional feet equal to 10\
times your Intelligence modifier. This movement\
doesn’t have to be in a straight line.\

Original layout: [[../../srd-5.2.1/pages/249|page 249]].
