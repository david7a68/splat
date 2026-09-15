---
splat_schema: 1
splat_id: 0cb8fa23-160f-5feb-9b95-50ceb950a9c7
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_13_studied_attacks
splat_name: 'Level 13: Studied Attacks'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_13_studied_attacks
  parameters:
    trigger: actor.attack_missed_creature
    grant:
      advantage: true
      scope: next_attack_against_same_creature
      expires: actor.next_turn.end
      consume_on: matching_attack_roll
    class_scope:
      class: fighter
      minimum_level: 13
  implementation: dnd2024:class_features_fighter_level_13_studied_attacks
splat_contracts:
- id: dnd2024:class_features_fighter_level_13_studied_attacks
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
  - attack.missed
  - attack.roll_pending
  - turn.end
  capabilities:
  - character_features
  - d20_tests
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_13_studied_attacks
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 67c5f4e677e4895c25885faf31b38e6622b6a03772ae2502a32b1eb20cdb103c
  parent_name: Fighter
  line_span:
    first: 4035
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 13: Studied Attacks You study your opponents and learn from each attack you make. If
    you make an attack roll against a creature and miss, you have Advantage on your next attack roll against
    that creature before the end of your next turn.'
  sha256: 20e45e74d811d4e872d4c4499f90079ed9f167c09472105d6fbaa3b33b4eebd7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 13: Studied Attacks

<!-- SRD page 48 -->

You study your opponents and learn from each at-\
tack you make. If you make an attack roll against a\
creature and miss, you have Advantage on your next\
attack roll against that creature before the end of\
your next turn.\
