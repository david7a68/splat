---
splat_schema: 1
splat_id: b7cef3b8-4a9e-549b-8d5b-0ab068c7a69e
splat_kind: item
splat_key: dnd2024:items/marvelous_pigments
splat_name: Marvelous Pigments
splat_origin:
  key: dnd2024:reference/magic_item/marvelous_pigments
splat_sources:
- title: SRD 5.2.1
  locator: pp. 230
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/marvelous_pigments
  implementation: dnd2024:item_marvelous_pigments
  parameters:
    box:
      initial_pots:
        expression: 1d4
        scope: use
      brush: true
      total_weight_lb: 1
    paint:
      cost_pots: 1
      brush_required: true
      duration:
        amount: 10
        unit: minute
      concentration: true
      user_must_remain_in_area: true
      area:
        shape: cube
        size_ft: 20
      elements:
        kinds:
        - three_dimensional_object
        - terrain_feature
        count: any
        all_extents_inside_area: true
      interrupted:
        painted_elements_vanish: true
        pot_wasted: true
      completion:
        elements_become_real: true
        door_connects_to_existing_beyond: true
        pit_depth_within_cube: true
      value_limits:
        per_object_gp: 25
        total_gp: 500
        exceeding:
          appearance: authentic
          material: worthless
          close_inspection_reveals: true
      painted_energy:
        dissipates_on_completion: true
        damage: 0
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_marvelous_pigments
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
  - painting.started
  - concentration.ended
  - position.changed
  - time.elapsed
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
  - spatial_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 54f56cf46e923210e808257de7fbd656f8b79cc92043b33951136600effa0d1b
  source_path: builtin/srd-5.2.1/magic_item/marvelous_pigments.md
  pages:
  - 230
  engine_gaps: &id001
  - dnd2024:item_marvelous_pigments
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: b610d4b2e20e5734ce18db04c3af8e6ebfb24f624dd277da6adde8ff139fdb3c
splat_clauses:
- key: source_rules
  text: '# Marvelous Pigments Wondrous Item, Very Rare This fine wooden box contains 1d4 pots of pigment
    and a brush (weighing 1 pound in total). Using the brush and expending 1 pot of pigment, you can paint
    any number of three-dimensional objects and terrain features (such as walls, doors, trees, flowers,
    weapons, webs, and pits), provided these elements are all confined to a 20-foot Cube. The effort takes
    10 minutes (regardless of the number of elements you create), during which time you must remain in
    the Cube, and requires Concentration. If your Concentration is broken or you leave the Cube before
    the work is done, all the painted elements vanish, and the pot of pigment is wasted. When the work
    is done, all the painted objects and terrain features become real. Thus, painting a door on a wall
    creates an actual door, which can be opened to whatever is beyond. Painting a pit creates a real pit,
    the entire depth of which must lie within the 20-foot Cube. No object created by a pot of pigment
    can have a value greater than 25 GP, and the total value of all objects created by a pot of pigment
    can’t exceed 500 GP. If you paint objects of greater value (such as a large pile of gold), they look
    authentic, but close inspection reveals they’re made from paste, cookies, or some other worthless
    material. If you paint a form of energy such as fire or lightning, the energy dissipates as soon as
    you complete the painting, doing no harm.'
  sha256: 2f725f1929b8c796adea4c54dbcb48e5bdf5d4d76cb1416b115c555afb7eb72a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Marvelous Pigments

<!-- SRD page 230 -->

Wondrous Item, Very Rare\
This fine wooden box contains 1d4 pots of pigment\
and a brush (weighing 1 pound in total).\
Using the brush and expending 1 pot of pigment,\
you can paint any number of three-dimensional\
objects and terrain features (such as walls, doors,\
trees, flowers, weapons, webs, and pits), provided\
these elements are all confined to a 20-foot Cube.\
The effort takes 10 minutes (regardless of the num-\
ber of elements you create), during which time you\
must remain in the Cube, and requires Concentra-\
tion. If your Concentration is broken or you leave\
the Cube before the work is done, all the painted\
elements vanish, and the pot of pigment is wasted.\
When the work is done, all the painted objects\
and terrain features become real. Thus, painting a\
door on a wall creates an actual door, which can be\
opened to whatever is beyond. Painting a pit creates\
a real pit, the entire depth of which must lie within\
the 20-foot Cube.\
No object created by a pot of pigment can have a\
value greater than 25 GP, and the total value of all\
objects created by a pot of pigment can’t exceed 500\
GP. If you paint objects of greater value (such as a\
large pile of gold), they look authentic, but close in-\
spection reveals they’re made from paste, cookies,\
or some other worthless material.\
If you paint a form of energy such as fire or light-\
ning, the energy dissipates as soon as you complete\
the painting, doing no harm.\

Original layout: [[../../srd-5.2.1/pages/230|page 230]].
