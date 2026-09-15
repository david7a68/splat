---
splat_schema: 1
splat_id: a3652a77-77b0-5fce-9a0a-ecff179361c8
splat_kind: rule_group
splat_key: dnd2024:species/halfling
splat_name: Halfling
splat_origin:
  key: dnd2024:reference/pages/086
splat_sources:
- title: SRD 5.2.1
  locator: pp. 86
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: species
splat_rule:
  mechanic: dnd2024:species/halfling
  implementation: dnd2024:species_halfling
  parameters:
    creature_type: humanoid
    size: small
    height_feet:
      typical_minimum: 2
      typical_maximum: 3
    speed_feet: 30
    brave:
      save_advantage:
        condition: frightened
        purposes:
        - avoid
        - end
    nimbleness:
      move_through_creature_space:
        minimum_size_difference: 1
      stop_in_occupied_space: false
    luck:
      trigger: actor.d20_test.d20_is_1
      optional_reroll: true
      must_use_new_roll: true
    naturally_stealthy:
      hide_obscurement_qualifier:
        only_obscured_by_creature: true
        minimum_size_difference: 1
splat_contracts:
- id: dnd2024:species_halfling
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - character.species_selected
  - saving_throw
  - participant.moved
  - d20_test.rolled
  - action.hide_pending
  capabilities:
  - character_features
  - conditions
  - d20_tests
  - movement
splat_conversion:
  status: specified
  category: species
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 86
  engine_gaps: &id001
  - dnd2024:species_halfling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 761939fce1e6640cae76085ab8d9c5c65095a5eda9f66158bb5ec604efebc496
  line_span:
    first: 7366
    count: 16
splat_clauses:
- key: source_rules
  text: '# Halfling Creature Type: Humanoid Size: Small (about 2–3 feet tall) Speed: 30 feet As a Halfling,
    you have these special traits. Brave. You have Advantage on saving throws you make to avoid or end
    the Frightened condition. Halfling Nimbleness. You can move through the space of any creature that
    is a size larger than you, but you can’t stop in the same space. Luck. When you roll a 1 on the d20
    of a D20 Test, you can reroll the die, and you must use the new roll. Naturally Stealthy. You can
    take the Hide action even when you are obscured only by a creature that is at least one size larger
    than you.'
  sha256: cf2976ec196bbe5e66a60adafd7904c5688691f7c8ed7e09b924d576bbaf8688
  status: contract_specified
splat_requires_mechanics: *id001
---

# Halfling

<!-- SRD page 86 -->

Creature Type: Humanoid\
Size: Small (about 2–3 feet tall)\
Speed: 30 feet\
As a Halfling, you have these special traits.\
Brave. You have Advantage on saving throws you\
make to avoid or end the Frightened condition.\
Halfling Nimbleness. You can move through the\
space of any creature that is a size larger than you,\
but you can’t stop in the same space.\
Luck. When you roll a 1 on the d20 of a D20 Test,\
you can reroll the die, and you must use the new\
roll.\
Naturally Stealthy. You can take the Hide action\
even when you are obscured only by a creature that\
is at least one size larger than you.\
