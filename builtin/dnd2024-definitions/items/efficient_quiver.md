---
splat_schema: 1
splat_id: 10512d84-4e76-5d69-ace0-3a78554c747a
splat_kind: item
splat_key: dnd2024:items/efficient_quiver
splat_name: Efficient Quiver
splat_origin:
  key: dnd2024:reference/magic_item/efficient_quiver
splat_sources:
- title: SRD 5.2.1
  locator: pp. 219
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/efficient_quiver
  implementation: dnd2024:item_efficient_quiver
  parameters:
    extradimensional_compartments:
    - capacity: 60
      item_shapes:
      - arrow
      - bolt
      - similar
    - capacity: 18
      item_shapes:
      - javelin
      - similar
    - capacity: 6
      item_shapes:
      - bow
      - quarterstaff
      - spear
      - similar_long_object
    maximum_total_weight_lb: 2
    retrieve_economy: normal_quiver_or_scabbard
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_efficient_quiver
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
  - inventory.insert_pending
  - inventory.retrieve_pending
  capabilities:
  - inventory
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 535f5d0ef34b2e5b036dfdd3a2aab6dcc8773330e200b933d86438d6d9dd8b20
  source_path: builtin/srd-5.2.1/magic_item/efficient_quiver.md
  pages:
  - 219
  engine_gaps: &id001
  - dnd2024:item_efficient_quiver
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 10ce1147b4b346fae17e16209cc14831aba57905bf44c07a1add00861416d6d7
splat_clauses:
- key: source_rules
  text: '# Efficient Quiver Wondrous Item, Uncommon Each of the quiver’s three compartments connects to
    an extradimensional space that allows the quiver to hold numerous items while never weighing more
    than 2 pounds. The shortest compartment can hold up to 60 Arrows, Bolts, or similar objects. The midsize
    compartment holds up to 18 Javelins or similar objects. The longest compartment holds up to 6 long
    objects, such as bows, Quarterstaffs, or Spears. You can draw any item the quiver contains as if doing
    so from a regular quiver or scabbard.'
  sha256: 830865429f8f96e40fde22df13eb76c3f1853c5154735036905f4d7a09c3d397
  status: contract_specified
splat_requires_mechanics: *id001
---

# Efficient Quiver

<!-- SRD page 219 -->

Wondrous Item, Uncommon\
Each of the quiver’s three compartments connects\
to an extradimensional space that allows the quiver\
to hold numerous items while never weighing more\
than 2 pounds. The shortest compartment can hold\
up to 60 Arrows, Bolts, or similar objects. The mid-\
size compartment holds up to 18 Javelins or similar\
objects. The longest compartment holds up to 6 long\
objects, such as bows, Quarterstaffs, or Spears.\
You can draw any item the quiver contains as if\
doing so from a regular quiver or scabbard.\

Original layout: [[../../srd-5.2.1/pages/219|page 219]].
