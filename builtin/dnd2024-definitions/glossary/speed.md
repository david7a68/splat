---
splat_schema: 1
splat_id: b912ddd6-07ec-5099-86e2-9187bed9f6eb
splat_kind: rule
splat_key: dnd2024:glossary/speed
splat_name: Speed
splat_origin:
  key: dnd2024:reference/glossary/speed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 188
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/speed
  parameters:
    unit: feet
    budget_period: own_turn
    special_modes:
    - burrow
    - climb
    - fly
    - swim
    mode_choice:
      chooser: subject
      switch_during_move: true
    on_mode_switch:
      remaining:
        op: subtract
        args:
        - var: new_speed
        - var: distance_already_moved
      usable_if_remaining_greater_than: 0
    temporary_speed_changes:
      also_apply_to_all_special_speeds: true
      same_duration: true
      additive_delta: same_amount
      set_zero: all_zero
      multiply: same_factor
  implementation: dnd2024:glossary_speed
splat_contracts:
- id: dnd2024:glossary_speed
  inputs:
    subject:
      type: participant
      required: true
    selected_mode:
      type: choice
      required: true
    distance_already_moved:
      type: number
      required: true
    speed_effect:
      required: false
      type: effect
  outputs:
  - remaining_movement
  - speeds
  events:
  - movement.pending
  - movement_mode.changed
  - speed.changed
  capabilities:
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 9a6775bff56fc5d47a68b8f29ca406a11e8d9229816e092df38af2252b1d168e
  source_path: builtin/srd-5.2.1/glossary/speed.md
  pages:
  - 188
  engine_gaps: &id001
  - dnd2024:glossary_speed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 734edbb0eb0f0865c5224e25fd5810bef6e14c8569c9750896e6e9f092a6ab69
splat_clauses:
- key: source_rules
  text: '# Speed A creature has a Speed, which is the distance in feet the creature can cover when it
    moves on its turn. See also “Climbing,” “Crawling,” “Flying,” “Jumping,” “Swimming” and “Playing the
    Game” (“Combat”). Special Speeds. Some creatures have special speeds, such as a Burrow Speed, Climb
    Speed, Fly Speed, or Swim Speed, each of which is defined in this glossary. If you have more than
    one speed, choose which one to use when you move; you can switch between the speeds during your move.
    Whenever you switch, subtract the distance already moved from the new speed. The result determines
    how much farther you can move. If the result is 0 or less, you can’t use the new speed during the
    current move. For example, if you have a Speed of 30 and a Fly Speed of 40, you could fly 10 feet,
    walk 10 feet, and leap into the air to fly 20 feet more. Changes to Your Speeds. If an effect increases
    or decreases your Speed for a time, any special speed you have increases or decreases by an equal
    amount for the same duration. For example, if your Speed is reduced to 0 and you have a Climb Speed,
    your Climb Speed is also reduced to 0. Similarly, if your Speed is halved and you have a Fly Speed,
    your Fly Speed is also halved.'
  sha256: c36e6640bd5fe1c1d4edf3c5a9f76aa58393375d5e21170a420cd8e98372bffd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Speed

<!-- SRD page 188 -->

A creature has a Speed, which is the distance in feet\
the creature can cover when it moves on its turn.\
See also “Climbing,” “Crawling,” “Flying,” “Jumping,”\
“Swimming” and “Playing the Game” (“Combat”).\
Special Speeds. Some creatures have special\
speeds, such as a Burrow Speed, Climb Speed, Fly\
Speed, or Swim Speed, each of which is defined\
in this glossary. If you have more than one speed,\
choose which one to use when you move; you can\
switch between the speeds during your move.\
Whenever you switch, subtract the distance already\
moved from the new speed. The result determines\
how much farther you can move. If the result is 0 or\
less, you can’t use the new speed during the current\
move. For example, if you have a Speed of 30 and a\
Fly Speed of 40, you could fly 10 feet, walk 10 feet,\
and leap into the air to fly 20 feet more.\
Changes to Your Speeds. If an effect increases or\
decreases your Speed for a time, any special speed\
you have increases or decreases by an equal amount\
for the same duration. For example, if your Speed\
is reduced to 0 and you have a Climb Speed, your\
Climb Speed is also reduced to 0. Similarly, if your\
Speed is halved and you have a Fly Speed, your Fly\
Speed is also halved.\

Original layout: [[../../srd-5.2.1/pages/188|page 188]].
