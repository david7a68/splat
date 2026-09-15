---
splat_schema: 1
splat_id: 8e4d8e47-b47e-5fce-abfc-f501e8ddac94
splat_kind: item
splat_key: dnd2024:items/bag_of_holding
splat_name: Bag of Holding
splat_origin:
  key: dnd2024:reference/magic_item/bag_of_holding
splat_sources:
- title: SRD 5.2.1
  locator: pp. 212
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  mass:
    value: 5
    unit: lb
  stackable: false
  equipment:
    allowed:
    - carried
    attunement: none
  containment:
    capacity:
      mass:
        value: 500
        unit: lb
      volume:
        value: 64
        unit: ft3
    carried_mass:
      value: 5
      unit: lb
splat_rule:
  mechanic: dnd2024:items/bag_of_holding
  implementation: dnd2024:item_bag_of_holding
  parameters:
    container:
      extradimensional: true
      interior_approximate_ft:
      - 2
      - 2
      - 4
      capacity_weight_lb: 500
      capacity_volume_cubic_ft: 64
      weight_lb: 5
      weight_independent_of_contents: true
      retrieve_action: utilize
      destruction_triggers:
      - overloaded
      - pierced
      - torn
      destruction_contents_destination:
        plane: astral
        distribution: scattered
      inverted:
        contents: spill_unharmed
        usable: false
        restore: turn_right_side_out
      air_supply:
        minutes: 10
        divide_by: breathing_creature_count
      interior_dimensions_are_approximate: true
    nested_extradimensional:
      trigger: placed_inside_item_created_extradimensional_space
      examples:
      - key: dnd2024:items/handy_haversack
      - key: dnd2024:items/portable_hole
      includes_similar_items: true
      destroy: both_items
      gate:
        origin: nesting_location
        shape: sphere
        radius_ft: 10
        targets: all_creatures_in_area
        destination:
          plane: astral
          location: random
        one_way: true
        closes_immediately_after_transport: true
        reopenable: false
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_bag_of_holding
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
  - item_state
  - locations
  - effects
  events:
  - item.used
  - item.damaged
  - item.placed
  capabilities:
  - inventory
  - extradimensional_spaces
  - planar_travel
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 6fc2ccc0d57ecd879628a1a0e7cf86d872f4b1a59b4fccff0d98a9f144b2c0a9
  source_path: builtin/srd-5.2.1/magic_item/bag_of_holding.md
  pages:
  - 212
  engine_gaps: &id001
  - dnd2024:item_bag_of_holding
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: d47d34ea09ed590c9ce5c3c4999c7db4b03c8ae85a6c7d2730b13addb14ae563
splat_clauses:
- key: source_rules
  text: '# Bag of Holding Wondrous Item, Uncommon This bag has an interior space considerably larger than
    its outside dimensions—roughly 2 feet square and 4 feet deep on the inside. The bag can hold up to
    500 pounds, not exceeding a volume of 64 cubic feet. The bag weighs 5 pounds, regardless of its contents.
    Retrieving an item from the bag requires a Utilize action. If the bag is overloaded, pierced, or torn,
    it is destroyed, and its contents are scattered in the Astral Plane. If the bag is turned inside out,
    its contents spill forth unharmed, but the bag must be put right before it can be used again. The
    bag holds enough air for 10 minutes of breathing, divided by the number of breathing creatures inside.
    Placing a Bag of Holding inside an extradimensional space created by a Handy Haversack, Portable Hole,
    or similar item instantly destroys both items and opens a gate to the Astral Plane. The gate originates
    where the one item was placed inside the other. Any creature within a 10-foot-radius Sphere centered
    on the gate is sucked through it to a random location on the Astral Plane. The gate then closes. The
    gate is one-way and can’t be reopened.'
  sha256: 998fc4ee8bb226dd1bfe0617493460e7d725b8262e870e0b39e467bf1e19857d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bag of Holding

<!-- SRD page 212 -->

Wondrous Item, Uncommon\
This bag has an interior space considerably larger\
than its outside dimensions—roughly 2 feet square\
and 4 feet deep on the inside. The bag can hold up\
to 500 pounds, not exceeding a volume of 64 cubic\
feet. The bag weighs 5 pounds, regardless of its\
contents. Retrieving an item from the bag requires\
a Utilize action.\
If the bag is overloaded, pierced, or torn, it is de-\
stroyed, and its contents are scattered in the Astral\
Plane. If the bag is turned inside out, its contents\
spill forth unharmed, but the bag must be put right\
before it can be used again. The bag holds enough\
air for 10 minutes of breathing, divided by the num-\
ber of breathing creatures inside.\
Placing a Bag of Holding inside an extradimen-\
sional space created by a Handy Haversack, Portable\
Hole, or similar item instantly destroys both items\
and opens a gate to the Astral Plane. The gate orig-\
inates where the one item was placed inside the\
other. Any creature within a 10-foot-radius Sphere\
centered on the gate is sucked through it to a ran-\
dom location on the Astral Plane. The gate then\
closes. The gate is one-way and can’t be reopened.\

Original layout: [[../../srd-5.2.1/pages/212|page 212]].
