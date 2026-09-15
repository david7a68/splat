---
splat_schema: 1
splat_id: 10682190-5b2d-5668-bc9a-5e9efe7ba78d
splat_kind: rule
splat_key: dnd2024:rules/poison_burnt_othur_fumes_500_gp
splat_name: Burnt Othur Fumes (500 GP)
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_burnt_othur_fumes_500_gp
  parameters:
    cost_gp: 500
    exposure_route: inhaled
    initial_save:
      ability: constitution
      dc: 13
    initial_failure:
      damage:
        dice:
          count: 3
          sides: 6
        average: 10
        type: poison
    repeat:
      timing: subject_turn_start
      ability: constitution
      dc: 13
      failure_damage:
        dice:
          count: 1
          sides: 6
        average: 3
        type: poison
      requires_initial_failure: false
    ends:
      successful_saves: 3
      consecutive_required: false
      count_initial_save_success: true
  implementation: dnd2024:rules_poison_burnt_othur_fumes_500_gp
splat_contracts:
- id: dnd2024:rules_poison_burnt_othur_fumes_500_gp
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - poison.exposure
  - turn.started
  capabilities:
  - poison
  - damage
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 197
  engine_gaps: &id001
  - dnd2024:rules_poison_burnt_othur_fumes_500_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: d0bdcc30ddca1e9cfebab2150da11c6dbc78be3f3528a5ac2920a21f2c1131e7
  parent_name: Poison
  line_span:
    first: 9832
    count: 9
splat_clauses:
- key: source_rules
  text: '# Burnt Othur Fumes (500 GP) Inhaled Poison A creature subjected to Burnt Othur Fumes must succeed
    on a DC 13 Constitution saving throw or take 10 (3d6) Poison damage, and it must repeat the save at
    the start of each of its turns. On each successive failed save, the creature takes 3 (1d6) Poison
    damage. After three successful saves, the poison ends.'
  sha256: 5fdd9b3cbe5670f76c9283e0f6a96425ca2c2a5749bbad57c639e7202ba9f88d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Burnt Othur Fumes (500 GP)

<!-- SRD page 197 -->

Inhaled Poison\
A creature subjected to Burnt Othur Fumes must\
succeed on a DC 13 Constitution saving throw or\
take 10 (3d6) Poison damage, and it must repeat the\
save at the start of each of its turns. On each succes-\
sive failed save, the creature takes 3 (1d6) Poison\
damage. After three successful saves, the poison\
ends.\
