---
splat_schema: 1
splat_id: f33d1574-3ff6-5ba9-b574-615ece66b436
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_13_improved_brutal_strike
splat_name: 'Level 13: Improved Brutal Strike'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_13_improved_brutal_strike
  parameters:
    extends:
      key: dnd2024:class_features/barbarian_level_9_brutal_strike
    options:
      staggering_blow:
        next_saving_throw: disadvantage
        opportunity_attacks: false
        opportunity_attack_prohibition_end: actor.next_turn.start
      sundering_blow:
        next_attack:
          attacker_not_actor: true
          target: struck_creature
          bonus: 5
          before: actor.next_turn.start
          maximum_stacking_instances: 1
    class_scope:
      class: barbarian
      minimum_level: 13
  implementation: dnd2024:class_features_barbarian_level_13_improved_brutal_strike
splat_contracts:
- id: dnd2024:class_features_barbarian_level_13_improved_brutal_strike
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
  - attack.hit
  - saving_throw
  - attack.roll_pending
  - turn.start
  capabilities:
  - character_features
  - d20_tests
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 30
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_13_improved_brutal_strike
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 40e60e1f573a84032c28f928c1312cc26fa65d7ca4359fd3513ab400e086e4d1
  parent_name: Barbarian
  line_span:
    first: 2413
    count: 12
splat_clauses:
- key: source_rules
  text: '# Level 13: Improved Brutal Strike You have honed new ways to attack furiously. The following
    effects are now among your Brutal Strike options. Staggering Blow. The target has Disadvantage on
    the next saving throw it makes, and it can’t make Opportunity Attacks until the start of your next
    turn. Sundering Blow. Before the start of your next turn, the next attack roll made by another creature
    against the target gains a +5 bonus to the roll. An attack roll can gain only one Sundering Blow bonus.'
  sha256: 327f867a0f8e1049a7c356543a765abb686069c67e10a6eebc0a93d608a5c620
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 13: Improved Brutal Strike

<!-- SRD page 30 -->

You have honed new ways to attack furiously. The\
following effects are now among your Brutal Strike\
options.\
Staggering Blow. The target has Disadvantage on\
the next saving throw it makes, and it can’t make\
Opportunity Attacks until the start of your next\
turn.\
Sundering Blow. Before the start of your next\
turn, the next attack roll made by another creature\
against the target gains a +5 bonus to the roll. An\
attack roll can gain only one Sundering Blow bonus.\
