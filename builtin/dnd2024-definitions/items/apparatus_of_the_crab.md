---
splat_schema: 1
splat_id: d1fb3766-2184-5bd4-80a3-51f2f795af83
splat_kind: item
splat_key: dnd2024:items/apparatus_of_the_crab
splat_name: Apparatus of the Crab
splat_origin:
  key: dnd2024:reference/magic_item/apparatus_of_the_crab
splat_sources:
- title: SRD 5.2.1
  locator: pp. 210
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/apparatus_of_the_crab
  implementation: dnd2024:item_apparatus_of_the_crab
  parameters:
    weight_lb: 500
    size: large
    ac: 20
    hp: 200
    damage_immunities:
    - poison
    - psychic
    hidden_catch:
      check:
        ability: intelligence
        skill: investigation
        dc: 20
      success: find_catch
      release_catch: unlock_entry_hatch
    occupants:
      maximum: 2
      sizes:
      - tiny
      - small
      - medium
    pilot_required: 1
    hatch_closed:
      airtight: true
      watertight: true
      breathing_hours:
        op: divide
        args:
        - 10
        - binding: breathing_occupants
      zero_breathing_occupants_air_depletion: false
    floats: true
    safe_depth_ft: 900
    below_safe_depth:
      interval:
        amount: 1
        unit: minute
      damage:
        roll:
          expression: 2d6
          scope: use
        type: bludgeoning
    legs_extended:
      speed_ft: 30
      swim_speed_ft: 30
    legs_retracted:
      speed_ft: 0
      swim_speed_ft: 0
      speed_bonuses_allowed: false
    operate:
      action: utilize
      actor_location: inside_compartment
      maximum_levers: 2
      positions:
      - up
      - down
      reset_after_each_use: neutral
    levers:
    - number: 1
      up:
        legs: extended
      down:
        legs: retracted
    - number: 2
      up:
        forward_window_shutter: open
      down:
        forward_window_shutter: closed
    - number: 3
      up:
        side_window_shutters: open
        count_per_side: 2
      down:
        side_window_shutters: closed
        count_per_side: 2
    - number: 4
      up:
        claws: extended
        count: 2
      down:
        claws: retracted
    - number: 5
      scope: each_extended_claw
      up:
        attack:
          kind: melee
          bonus: 8
          reach_ft: 5
          hit:
            damage:
              average: 7
              roll:
                expression: 2d6
                scope: attack
              type: bludgeoning
      down:
        attack:
          kind: melee
          bonus: 8
          reach_ft: 5
          hit:
            condition: grappled
            escape_dc: 15
    - number: 6
      requires: legs_extended
      up:
        movement: forward
        modes:
        - walk
        - swim
      down:
        movement: backward
        modes:
        - walk
        - swim
    - number: 7
      requires: legs_extended
      up:
        turn_degrees: 90
        direction: counterclockwise
      down:
        turn_degrees: 90
        direction: clockwise
    - number: 8
      up:
        light:
          bright_radius_ft: 30
          dim_additional_ft: 30
      down:
        light: 'off'
    - number: 9
      requires: in_liquid
      up:
        vertical_movement: sink
        maximum_ft: 20
      down:
        vertical_movement: rise
        maximum_ft: 20
    - number: 10
      up:
        rear_hatch: open_unsealed
      down:
        rear_hatch: closed_sealed
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_apparatus_of_the_crab
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
  - action.utilize_pending
  - time.minute
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
  - spatial_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 1d23eeef5b946474d413a9e7ef96916d840cf1235f9157de2b87910479a9c0bd
  source_path: builtin/srd-5.2.1/magic_item/apparatus_of_the_crab.md
  pages:
  - 210
  engine_gaps: &id001
  - dnd2024:item_apparatus_of_the_crab
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 0499e227719bcf0186eb28292529baf0a57fa98adc89ab63359ad663b3696531
splat_clauses:
- key: source_rules
  text: '# Apparatus of the Crab Wondrous Item, Legendary This item first appears to be a sealed iron
    barrel weighing 500 pounds. The barrel has a hidden catch, which can be found with a successful DC
    20 Intelligence (Investigation) check. Releasing the catch unlocks a hatch at one end of the barrel,
    allowing two Medium or smaller creatures to crawl inside. Ten levers are set in a row at the far end,
    each in a neutral position, able to move up or down. When certain levers are used, the apparatus transforms
    to resemble a giant lobster. The Apparatus of the Crab is a Large object with the following statistics:
    AC 20; HP 200; Speed 30 ft., Swim 30 ft. (or 0 ft. for both if the legs aren’t extended); Immunity
    to Poison and Psychic damage. To be used as a vehicle, the apparatus requires one pilot. While the
    apparatus’s hatch is closed, the compartment is airtight and watertight. The compartment holds enough
    air for 10 hours of breathing, divided by the number of breathing creatures inside. The apparatus
    floats on water. It can also go underwater to a depth of 900 feet. Below that, the vehicle takes 2d6
    Bludgeoning damage each minute from pressure. A creature in the compartment can take a Utilize action
    to move as many as two of the apparatus’s levers up or down. After each use, a lever goes back to
    its neutral position. Each lever, from left to right, functions as shown in the Apparatus of the Crab
    Levers table. Apparatus of the Crab Levers Lever Up 1 Legs extend, allowing the apparatus to walk
    and swim. Legs retract, reducing the apparatus’s Speed and Swim 2 Forward window shutter opens. 3
    Side window shutters open (two per side). 4 Two claws extend from the front side of the apparatus.
    5 Each extended claw makes the following melee attack: +8 to hit, reach 5 ft. Hit: 7 (2d6) Bludgeoning
    damage. 6 The apparatus walks or swims forward provided its legs are extended. 7 The apparatus turns
    90 degrees counterclockwise provided its legs are extended. 8 Eyelike fixtures emit Bright Light in
    a 30-foot radius and Dim Light for an additional 30 feet. 9 The apparatus sinks up to 20 feet if it’s
    in liquid. 10 The rear hatch unseals and opens.'
  sha256: 8f5c056c6b0210178b532756c10808abb93492f587154fdaaca28264670c0cd5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Apparatus of the Crab

<!-- SRD page 210 -->

Wondrous Item, Legendary\
This item first appears to be a sealed iron barrel\
weighing 500 pounds. The barrel has a hidden\
catch, which can be found with a successful DC 20\
Intelligence (Investigation) check. Releasing the\
catch unlocks a hatch at one end of the barrel, al-\
lowing two Medium or smaller creatures to crawl\
inside. Ten levers are set in a row at the far end,\
each in a neutral position, able to move up or down.\
When certain levers are used, the apparatus trans-\
forms to resemble a giant lobster.\
The Apparatus of the Crab is a Large object with\
the following statistics: AC 20; HP 200; Speed 30\
ft., Swim 30 ft. (or 0 ft. for both if the legs aren’t ex-\
tended); Immunity to Poison and Psychic damage.\
To be used as a vehicle, the apparatus requires\
one pilot. While the apparatus’s hatch is closed, the\
compartment is airtight and watertight. The com-\
partment holds enough air for 10 hours of breath-\
ing, divided by the number of breathing creatures\
inside.\
The apparatus floats on water. It can also go\
underwater to a depth of 900 feet. Below that, the\
vehicle takes 2d6 Bludgeoning damage each minute\
from pressure.\
A creature in the compartment can take a Utilize\
action to move as many as two of the apparatus’s\
levers up or down. After each use, a lever goes back\
to its neutral position. Each lever, from left to right,\
functions as shown in the Apparatus of the Crab\
Levers table.\
Apparatus of the Crab Levers\
Lever Up\
1 Legs extend, allowing the apparatus to walk and swim. Legs retract, reducing the apparatus’s Speed and Swim\
2 Forward window shutter opens.\
3 Side window shutters open (two per side).\
4 Two claws extend from the front side of the\
apparatus.\
5 Each extended claw makes the following melee attack:\
+8 to hit, reach 5 ft. Hit: 7 (2d6) Bludgeoning damage.\
6 The apparatus walks or swims forward provided its\
legs are extended.\
7 The apparatus turns 90 degrees counterclockwise\
provided its legs are extended.\
8 Eyelike fixtures emit Bright Light in a 30-foot radius\
and Dim Light for an additional 30 feet.\
9 The apparatus sinks up to 20 feet if it’s in liquid.\
10 The rear hatch unseals and opens.\

Original layout: [[../../srd-5.2.1/pages/210|page 210]].
