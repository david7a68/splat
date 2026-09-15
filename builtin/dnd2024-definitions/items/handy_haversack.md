---
splat_schema: 1
splat_id: f1a6257f-6147-50ed-9212-ee1a407bf2f1
splat_kind: item
splat_key: dnd2024:items/handy_haversack
splat_name: Handy Haversack
splat_origin:
  key: dnd2024:reference/magic_item/handy_haversack
splat_sources:
- title: SRD 5.2.1
  locator: pp. 224
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/handy_haversack
  implementation: dnd2024:item_handy_haversack
  parameters:
    weight_lb: 5
    weight_independent_of_contents: true
    pouches:
    - name: side
      count: 2
      capacity_lb: 200
      capacity_cubic_ft: 25
    - name: central
      count: 1
      capacity_lb: 500
      capacity_cubic_ft: 64
    extradimensional: true
    retrieve:
      action_choice:
      - utilize
      - bonus_action
      requested_item_position: top
    destroy_on:
    - pouch_overload
    - pouch_pierced
    - pouch_torn
    destroyed_contents:
      non_artifacts: lost_forever
      artifacts: reappear_elsewhere
    inside_out:
      contents: spill_unharmed
      usable: false
      restore: turn_right_side_out
    breath_per_pouch:
      minutes:
        divide:
        - 10
        - number_of_breathing_creatures
    extradimensional_nesting:
      trigger: placed_inside_extradimensional_item_space
      examples:
      - key: dnd2024:items/bag_of_holding
      - key: dnd2024:items/portable_hole
      destroy:
      - this_item
      - containing_item
      gate:
        origin: insertion_location
        targets: creatures
        radius_ft: 10
        total_cover_blocks: true
        destination:
          plane: astral
          location: random
        one_way: true
        close_after_transport: true
        can_reopen: false
    attunement_required: false
splat_contracts:
- id: dnd2024:item_handy_haversack
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
  - item.contents_changed
  - item.damaged
  - item.destroyed
  - item.turned_inside_out
  - item.activation_requested
  - time.elapsed
  capabilities:
  - item_effects
  - planar_travel
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 2201dd9c87beb2c0d3c4e2c7d12b841bb9a1b1b552e7604e7e7c7574ce20bad7
  source_path: builtin/srd-5.2.1/magic_item/handy_haversack.md
  pages:
  - 224
  engine_gaps: &id001
  - dnd2024:item_handy_haversack
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 523aa62e733feac905d2c19fcedb01519699422e18cefa7d68696bb43c098df8
splat_clauses:
- key: source_rules
  text: '# Handy Haversack Wondrous Item, Rare This backpack has a central pouch and two side pouches,
    each of which is an extradimensional space. Each side pouch can hold up to 200 pounds of material,
    not exceeding a volume of 25 cubic feet. The central pouch can hold up to 500 pounds of material,
    not exceeding a volume of 64 cubic feet. The haversack always weighs 5 pounds, regardless of its contents.
    Retrieving an item from the haversack requires a Utilize action or a Bonus Action (your choice). When
    you reach into the haversack for a specific item, the item is always magically on top. If any of its
    pouches is overloaded, pierced, or torn, the haversack ruptures and is destroyed. If the haversack
    is destroyed, its contents are lost forever, although an Artifact always turns up again somewhere.
    If the haversack is turned inside out, its contents spill forth unharmed, and the haversack must be
    put right before it can be used again. Each pouch of the haversack holds enough air for 10 minutes
    of breathing, divided by the number of breathing creatures inside. Placing the haversack inside an
    extradimensional space created by a Bag of Holding, Portable Hole, or similar item instantly destroys
    both items and opens a gate to the Astral Plane. The gate originates where the one item was placed
    inside the other. Any creature within 10 feet of the gate and not behind Total Cover is sucked through
    it and deposited in a random location on the Astral Plane. The gate then closes. The gate is one-way
    only and can’t be reopened.'
  sha256: 2a6795d78324d0e3b4d955d6db9d01e44cd799f738e375fcc230bf5c425ceead
  status: contract_specified
splat_requires_mechanics: *id001
---

# Handy Haversack

<!-- SRD page 224 -->

Wondrous Item, Rare\
This backpack has a central pouch and two side\
pouches, each of which is an extradimensional\
space. Each side pouch can hold up to 200 pounds\
of material, not exceeding a volume of 25 cubic feet.\
The central pouch can hold up to 500 pounds of ma-\
terial, not exceeding a volume of 64 cubic feet. The\
haversack always weighs 5 pounds, regardless of its\
contents.\
Retrieving an item from the haversack requires a\
Utilize action or a Bonus Action (your choice). When\
you reach into the haversack for a specific item, the\
item is always magically on top.\
If any of its pouches is overloaded, pierced, or\
torn, the haversack ruptures and is destroyed. If the\
haversack is destroyed, its contents are lost forever,\
although an Artifact always turns up again some-\
where. If the haversack is turned inside out, its con-\
tents spill forth unharmed, and the haversack must\
be put right before it can be used again.\
Each pouch of the haversack holds enough air for\
10 minutes of breathing, divided by the number of\
breathing creatures inside.\
Placing the haversack inside an extradimensional\
space created by a Bag of Holding, Portable Hole,\
or similar item instantly destroys both items and\
opens a gate to the Astral Plane. The gate originates\
where the one item was placed inside the other. Any\
creature within 10 feet of the gate and not behind\
Total Cover is sucked through it and deposited in\
a random location on the Astral Plane. The gate\
then closes. The gate is one-way only and can’t be\
reopened.\

Original layout: [[../../srd-5.2.1/pages/224|page 224]].
