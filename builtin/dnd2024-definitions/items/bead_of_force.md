---
splat_schema: 1
splat_id: c2d1bac7-a3a3-56c6-9f6b-1acc90a0d9b6
splat_kind: item
splat_key: dnd2024:items/bead_of_force
splat_name: Bead of Force
splat_origin:
  key: dnd2024:reference/magic_item/bead_of_force
splat_sources:
- title: SRD 5.2.1
  locator: pp. 212, 213
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/bead_of_force
  implementation: dnd2024:item_bead_of_force
  parameters:
    diameter_inches: 0.75
    weight_oz: 1
    found_quantity:
      roll:
        count: 1
        sides: 4
        add: 4
    action: magic
    throw_range_ft: 60
    consumed_on: impact
    area:
      shape: sphere
      radius_ft: 10
    save:
      ability: dexterity
      dc: 15
      failure:
        damage:
          amount:
            roll:
              count: 5
              sides: 4
          type: force
      success:
        damage: 0
    barrier:
      duration:
        amount: 1
        unit: minutes
      transparent: true
      trapped:
        save: failure
        wholly_inside: true
      push_out:
        or:
        - save: success
        - partially_inside: true
        direction: away_from_center
        until: outside
      passable:
      - breathable_air
      blocks:
      - attacks
      - other_effects
      - other_matter
      movement:
        actor: enclosed_creature
        action: utilize
        maximum_distance:
          multiply:
          - actor.speed
          - 0.5
      can_be_picked_up: true
      weight_lb: 1
      weight_includes_contents: true
    attunement_required: false
splat_contracts:
- id: dnd2024:item_bead_of_force
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
  - item.impact
  - time.elapsed
  capabilities:
  - item_effects
  - dice
  - effect_duration
  - spatial_targeting
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: f3b17f5f3c8487ed1ac2e682c2570f6b94e9bc2723ed2fe30c27e59848f98dcf
  source_path: builtin/srd-5.2.1/magic_item/bead_of_force.md
  pages:
  - 212
  - 213
  engine_gaps: &id001
  - dnd2024:item_bead_of_force
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 84ca90c5fcdbbb50b3b313fbf69adf4ee7ae8b96cc1914c0f77e75cdbb06fe03
splat_clauses:
- key: source_rules
  text: '# Bead of Force Wondrous Item, Rare This small black sphere measures 3/4 of an inch in diameter
    and weighs an ounce. Typically, 1d4 + 4 Beads of Force are found together. You can take a Magic action
    to throw the bead up to 60 feet. The bead explodes in a 10-foot-radius Sphere on impact and is destroyed.
    Each creature in the Sphere must succeed on a DC 15 Dexterity saving throw or take 5d4 Force damage.
    A sphere of transparent force then encloses the area for 1 minute. Any creature that failed the save
    and is completely within the area is trapped inside this sphere. Creatures that succeeded on the save
    or are partially within the area are pushed away from the center of the sphere until they are no longer
    inside it. Only breathable air can pass through the sphere’s wall. No attack or other effect can pass
    through. An enclosed creature can take a Utilize action to push against the sphere’s wall, moving
    the sphere up to half the creature’s Speed. The sphere can be picked up, and its magic causes it to
    weigh only 1 pound, regardless of the weight of creatures inside.'
  sha256: 4c11eecfdfb571fe8a396314908862990f293c3d9d365704dd855b2e3cea845b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bead of Force

<!-- SRD page 212 -->

Wondrous Item, Rare\
This small black sphere measures 3/4 of an inch in\
diameter and weighs an ounce. Typically, 1d4 + 4\
Beads of Force are found together.\
You can take a Magic action to throw the bead up\
to 60 feet. The bead explodes in a 10-foot-radius\
Sphere on impact and is destroyed. Each creature\

<!-- SRD page 213 -->

in the Sphere must succeed on a DC 15 Dexterity\
saving throw or take 5d4 Force damage. A sphere\
of transparent force then encloses the area for 1\
minute. Any creature that failed the save and is\
completely within the area is trapped inside this\
sphere. Creatures that succeeded on the save or are\
partially within the area are pushed away from the\
center of the sphere until they are no longer inside\
it. Only breathable air can pass through the sphere’s\
wall. No attack or other effect can pass through.\
An enclosed creature can take a Utilize action to\
push against the sphere’s wall, moving the sphere\
up to half the creature’s Speed. The sphere can be\
picked up, and its magic causes it to weigh only 1\
pound, regardless of the weight of creatures inside.\

Original layout: [[../../srd-5.2.1/pages/212|page 212]], [[../../srd-5.2.1/pages/213|page 213]].
