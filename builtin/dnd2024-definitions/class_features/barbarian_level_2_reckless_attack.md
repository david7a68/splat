---
splat_schema: 1
splat_id: 0af128d9-a737-5412-b69f-21628b0ba7b2
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_2_reckless_attack
splat_name: 'Level 2: Reckless Attack'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_2_reckless_attack
  parameters:
    optional: true
    trigger: actor.turn.first_attack_roll
    duration_end: actor.next_turn.start
    advantage:
      actor_attacks:
        ability: strength
      attacks_against_actor: true
    class_scope:
      class: barbarian
      minimum_level: 2
  implementation: dnd2024:class_features_barbarian_level_2_reckless_attack
splat_contracts:
- id: dnd2024:class_features_barbarian_level_2_reckless_attack
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
  - 29
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_2_reckless_attack
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 2d30fe055d85598a77a64119f55d5e11734a36e35cfea4775eebac2f93f037f8
  parent_name: Barbarian
  line_span:
    first: 2341
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 2: Reckless Attack You can throw aside all concern for defense to attack with increased
    ferocity. When you make your first attack roll on your turn, you can decide to attack recklessly.
    Doing so gives you Advantage on attack rolls using Strength until the start of your next turn, but
    attack rolls against you have Advantage during that time.'
  sha256: ea7cb4369abe0c1ec4bff37f0baead54bae9c508f72c16a7bd0c2b46f4e4663c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Reckless Attack

<!-- SRD page 29 -->

You can throw aside all concern for defense to at-\
tack with increased ferocity. When you make your\
first attack roll on your turn, you can decide to at-\
tack recklessly. Doing so gives you Advantage on at-\
tack rolls using Strength until the start of your next\
turn, but attack rolls against you have Advantage\
during that time.\
