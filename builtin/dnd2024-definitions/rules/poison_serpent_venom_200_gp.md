---
splat_schema: 1
splat_id: 8ed010ee-3518-578a-a264-830b49c0f933
splat_kind: rule
splat_key: dnd2024:rules/poison_serpent_venom_200_gp
splat_name: Serpent Venom (200 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_serpent_venom_200_gp
  parameters:
    poison:
      type: injury
      cost_gp: 200
    save:
      ability: constitution
      dc: 11
    failure:
      damage:
        dice:
          count: 3
          sides: 6
          add: 0
        average: 10
        type: poison
    success:
      damage_multiplier: 0.5
      round: down
  implementation: dnd2024:rules_poison_serpent_venom_200_gp
splat_contracts:
- id: dnd2024:rules_poison_serpent_venom_200_gp
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    hazard:
      type: object
      required: true
    event:
      type: event
      required: true
    resolution:
      type: resolution
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - effects
  - hazard_updates
  events: []
  capabilities:
  - hazards
  - effects
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 198
  engine_gaps: &id001
  - dnd2024:rules_poison_serpent_venom_200_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 4044a79dc4c97c451c77f5a26839225cc163538e35d0e8180e876809f5659931
  parent_name: Poison
  line_span:
    first: 9899
    count: 6
splat_clauses:
- key: source_rules
  text: '# Serpent Venom (200 GP) Injury Poison A creature subjected to Serpent Venom must succeed on
    a DC 11 Constitution saving throw, taking 10 (3d6) Poison damage on a failed save or half as much
    damage on a successful one.'
  sha256: c9d1b4cb85e2c5fd7b9466930d8d372b4bdec114fc889cf56e6bd8df0b03f44e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Serpent Venom (200 GP)

<!-- SRD page 198 -->

Injury Poison\
A creature subjected to Serpent Venom must suc-\
ceed on a DC 11 Constitution saving throw, taking\
10 (3d6) Poison damage on a failed save or half as\
much damage on a successful one.\
