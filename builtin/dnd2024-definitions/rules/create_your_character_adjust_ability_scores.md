---
splat_schema: 1
splat_id: 0f4a167b-d8b1-5a45-895b-1a23bae028e3
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_adjust_ability_scores
splat_name: Adjust Ability Scores
splat_origin:
  key: dnd2024:reference/pages/021
splat_sources:
- title: SRD 5.2.1
  locator: pp. 21
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_adjust_ability_scores
  parameters:
    ability_choices: background_three_abilities
    options:
    - increases:
      - 2
      - 1
      distinct_abilities: true
    - increases:
      - 1
      - 1
      - 1
      distinct_abilities: true
    score_maximum: 20
    timing: after_base_assignment
  implementation: dnd2024:rule_create_your_character_adjust_ability_scores
splat_contracts:
- id: dnd2024:rule_create_your_character_adjust_ability_scores
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - character_changes
  events: []
  capabilities:
  - character_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 21
  engine_gaps: &id001
  - dnd2024:rule_create_your_character_adjust_ability_scores
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: cb0ed36ee5e7427db26f7c901fce289a073b89a3b33cfb4b30eeb4bf631ee925
  parent_name: Create Your Character
  line_span:
    first: 1558
    count: 8
splat_clauses:
- key: source_rules
  text: '# Adjust Ability Scores After assigning your ability scores, adjust them according to your background.
    Your background lists three abilities; increase one of those scores by 2 and a different one by 1,
    or increase all three by 1. None of these increases can raise a score above 20. Some players like
    to increase their class’s primary ability, while others prefer to increase a low score.'
  sha256: 854799ff30cc51eed2eeb5e798177e78dd9bdbf5054212a26660e274fb2002de
  status: contract_specified
splat_requires_mechanics: *id001
---

# Adjust Ability Scores

<!-- SRD page 21 -->

After assigning your ability scores, adjust them ac-\
cording to your background. Your background lists\
three abilities; increase one of those scores by 2 and\
a different one by 1, or increase all three by 1. None\
of these increases can raise a score above 20.\
Some players like to increase their class’s primary\
ability, while others prefer to increase a low score.\
