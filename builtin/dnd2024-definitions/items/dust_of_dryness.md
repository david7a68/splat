---
splat_schema: 1
splat_id: c3f9a0ea-e00b-5f87-b496-b9e9185450a7
splat_kind: item
splat_key: dnd2024:items/dust_of_dryness
splat_name: Dust of Dryness
splat_origin:
  key: dnd2024:reference/magic_item/dust_of_dryness
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
  mechanic: dnd2024:items/dust_of_dryness
  implementation: dnd2024:item_dust_of_dryness
  parameters:
    pinches:
      initial:
        dice: 1
        sides: 6
        bonus: 4
      recovery: null
    water_absorption:
      action: utilize
      cost_pinches: 1
      maximum_cube_side_ft: 15
      output:
        kind: pellet
        size: marble
        weight: negligible
        placement: near_sprinkled_dust
        floats_or_rests: true
        stores: absorbed_water
      release:
        action: utilize
        requires: smash_against_hard_surface
        water: all_stored
        destroy_pellet: true
        end_magic: true
    elemental_damage:
      action: utilize
      cost_pinches: 1
      range_ft: 5
      target:
        type: elemental
        composition: mostly_water
      save:
        ability: constitution
        dc: 13
      damage:
        roll:
          dice: 10
          sides: 6
          bonus: 0
        type: necrotic
        success_multiplier: 0.5
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_dust_of_dryness
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
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - consumables
  - effects
  - item_resources
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 84c72ba0d0184aefdc8fd5d4a6791525374bfa740cd61a754e51ecaf3cf4d522
  source_path: builtin/srd-5.2.1/magic_item/dust_of_dryness.md
  pages:
  - 219
  engine_gaps: &id001
  - dnd2024:item_dust_of_dryness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 08d4b0c1b14148ed31ba717b6d7fa5dfd036819952129fdaa2ad4216e455435c
splat_clauses:
- key: source_rules
  text: '# Dust of Dryness Wondrous Item, Uncommon This small packet contains 1d6 + 4 pinches of dust.
    As a Utilize action, you can sprinkle a pinch of the dust over water, turning up to a 15-foot Cube
    of water into one marble-sized pellet, which floats or rests near where the dust was sprinkled. The
    pellet’s weight is negligible. A creature can take a Utilize action to smash the pellet against a
    hard surface, causing the pellet to shatter and release the water the dust absorbed. Doing so destroys
    the pellet and ends its magic. As a Utilize action, you can sprinkle a pinch of the dust on an Elemental
    within 5 feet of yourself that is composed mostly of water (such as a Water Elemental). Such a creature
    exposed to a pinch of the dust makes a DC 13 Constitution saving throw, taking 10d6 Necrotic damage
    on a failed save or half as much damage on a successful one.'
  sha256: 4a5eb463927b5a817fa09ada1c0d9a0f5df3c21e9f0375c54788f1b987572690
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dust of Dryness

<!-- SRD page 219 -->

Wondrous Item, Uncommon\
This small packet contains 1d6 + 4 pinches of dust.\
As a Utilize action, you can sprinkle a pinch of the\
dust over water, turning up to a 15-foot Cube of\
water into one marble-sized pellet, which floats\
or rests near where the dust was sprinkled. The\
pellet’s weight is negligible. A creature can take\
a Utilize action to smash the pellet against a hard\
surface, causing the pellet to shatter and release the\
water the dust absorbed. Doing so destroys the pel-\
let and ends its magic.\
As a Utilize action, you can sprinkle a pinch of the\
dust on an Elemental within 5 feet of yourself that\
is composed mostly of water (such as a Water Ele-\
mental). Such a creature exposed to a pinch of the\
dust makes a DC 13 Constitution saving throw, tak-\
ing 10d6 Necrotic damage on a failed save or half as\
much damage on a successful one.\

Original layout: [[../../srd-5.2.1/pages/219|page 219]].
