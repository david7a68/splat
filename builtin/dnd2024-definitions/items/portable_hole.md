---
splat_schema: 1
splat_id: 35b2e639-26f3-5173-9847-68f8312e20c2
splat_kind: item
splat_key: dnd2024:items/portable_hole
splat_name: Portable Hole
splat_origin:
  key: dnd2024:reference/magic_item/portable_hole
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234, 235
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/portable_hole
  implementation: dnd2024:item_portable_hole
  parameters:
    unfolded_diameter_ft: 6
    open:
      action: magic
      place_on_or_against: solid_surface
      space:
        shape: cylinder
        diameter_ft: 6
        depth_ft: 10
        plane: extradimensional
      creates_through_passage: false
      exit: climb_out
    close:
      action: magic
      grasp_edges_and_fold: true
      retain_contents: true
    weight: negligible_regardless_of_contents
    closed:
      escape:
        action: action
        check:
          ability: strength
          skill: athletics
          dc: 10
        success:
          exit_distance_ft: 5
      breathing_hours:
        op: divide
        args:
        - 1
        - binding: breathing_occupants
      zero_breathing_occupants_air_depletion: false
    nested_extradimensional:
      trigger: placed_inside_extradimensional_item
      examples:
      - key: dnd2024:items/bag_of_holding
      - key: dnd2024:items/handy_haversack
      include_similar_items: true
      destroy_both_items: true
      gate:
        origin: insertion_location
        destination_plane: astral
        destination_location: random
        creatures_radius_ft: 10
        exclude_total_cover: true
        forced_transport: true
        one_way: true
        close_after_transport: true
        reopenable: false
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_portable_hole
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
    breathing_occupants:
      required: false
      type: integer
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - action.magic_pending
  - action.pending
  - inventory.insert_pending
  - time.elapsed
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
  - spatial_state
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 45ab6089b1af6c6d00d8af7d7b8d6ef5e20070669b4dd7852cd2f04490f370ca
  source_path: builtin/srd-5.2.1/magic_item/portable_hole.md
  pages:
  - 234
  - 235
  engine_gaps: &id001
  - dnd2024:item_portable_hole
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: a00b68735ae35e6d0cd66d5f56359e73a9e0f5e9c21072ff3d2b70c7102fbf12
splat_clauses:
- key: source_rules
  text: '# Portable Hole Wondrous Item, Rare This fine black cloth, soft as silk, is folded up to the
    dimensions of a handkerchief. It unfolds into a circular sheet 6 feet in diameter. You can take a
    Magic action to unfold a Portable Hole and place it on or against a solid surface, whereupon the Portable
    Hole creates an extradimensional hole 10 feet deep. The cylindrical space within the hole exists on
    a different plane of existence, so it can’t be used to create open passages. Any creature inside an
    open Portable Hole can exit the hole by climbing out of it. You can take a Magic action to close a
    Portable Hole by taking hold of the edges of the cloth and folding it up. Folding the cloth closes
    the hole, and any creatures or objects within remain in the extradimensional space. No matter what’s
    in it, the hole weighs next to nothing. If the hole is folded up, a creature within the hole’s extradimensional
    space can take an action to make a DC 10 Strength (Athletics) check. On a successful check, the creature
    forces its way out and appears within 5 feet of the Portable Hole. A closed Portable Hole holds enough
    air for 1 hour of breathing, divided by the number of breathing creatures inside. Placing a Portable
    Hole inside an extradimensional space created by a Bag of Holding, Handy Haversack, or similar item
    instantly destroys both items and opens a gate to the Astral Plane. The gate originates where the
    one item was placed inside the other. Any creature within 10 feet of the gate and not behind Total
    Cover is sucked through it and deposited in a random location on the Astral Plane. The gate then closes.
    The gate is one-way only and can’t be reopened.'
  sha256: 0ab4a89ff29902ea4873acc40140f976834738c83c0dfceec759c2f0c1af5ae0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Portable Hole

<!-- SRD page 234 -->

Wondrous Item, Rare\
This fine black cloth, soft as silk, is folded up to the\
dimensions of a handkerchief. It unfolds into a cir-\
cular sheet 6 feet in diameter.\
You can take a Magic action to unfold a Por-\
table Hole and place it on or against a solid sur-\
face, whereupon the Portable Hole creates an\

<!-- SRD page 235 -->

extradimensional hole 10 feet deep. The cylindrical\
space within the hole exists on a different plane\
of existence, so it can’t be used to create open pas-\
sages. Any creature inside an open Portable Hole\
can exit the hole by climbing out of it.\
You can take a Magic action to close a Portable\
Hole by taking hold of the edges of the cloth and\
folding it up. Folding the cloth closes the hole, and\
any creatures or objects within remain in the ex-\
tradimensional space. No matter what’s in it, the\
hole weighs next to nothing.\
If the hole is folded up, a creature within the hole’s\
extradimensional space can take an action to make\
a DC 10 Strength (Athletics) check. On a successful\
check, the creature forces its way out and appears\
within 5 feet of the Portable Hole. A closed Portable\
Hole holds enough air for 1 hour of breathing, di-\
vided by the number of breathing creatures inside.\
Placing a Portable Hole inside an extradimensional\
space created by a Bag of Holding, Handy Haversack,\
or similar item instantly destroys both items and\
opens a gate to the Astral Plane. The gate originates\
where the one item was placed inside the other. Any\
creature within 10 feet of the gate and not behind\
Total Cover is sucked through it and deposited in\
a random location on the Astral Plane. The gate\
then closes. The gate is one-way only and can’t be\
reopened.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]], [[../../srd-5.2.1/pages/235|page 235]].
