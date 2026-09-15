---
splat_schema: 1
splat_id: c338b33d-31da-5fb5-ac44-6a7c66b3718e
splat_kind: rule
splat_key: dnd2024:rules/poison_assassin_s_blood_150_gp
splat_name: Assassin’s Blood (150 GP)
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_assassin_s_blood_150_gp
  parameters:
    poison:
      type: ingested
      cost_gp: 150
    save:
      ability: constitution
      dc: 10
    failure:
      damage:
        dice:
          count: 1
          sides: 12
          add: 0
        average: 6
        type: poison
      condition:
        id: poisoned
        duration:
          value: 24
          unit: hours
    success:
      damage_multiplier: 0.5
      round: down
      condition: null
  implementation: dnd2024:rules_poison_assassin_s_blood_150_gp
splat_contracts:
- id: dnd2024:rules_poison_assassin_s_blood_150_gp
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
  - 197
  engine_gaps: &id001
  - dnd2024:rules_poison_assassin_s_blood_150_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: d739aa1467da1cbb65adc94e6db9d5d48be769d2d04879d8419ea4217605b47b
  parent_name: Poison
  line_span:
    first: 9825
    count: 7
splat_clauses:
- key: source_rules
  text: '# Assassin’s Blood (150 GP) Ingested Poison A creature subjected to Assassin’s Blood makes a
    DC 10 Constitution saving throw. On a failed save, the creature takes 6 (1d12) Poison damage and has
    the Poisoned condition for 24 hours. On a successful save, the creature takes half as much damage
    only.'
  sha256: 18d406f8e4ca4e0dfa7e4f806758d956cf68d4d1e210482a7b902c2e4a66f07d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Assassin’s Blood (150 GP)

<!-- SRD page 197 -->

Ingested Poison\
A creature subjected to Assassin’s Blood makes a\
DC 10 Constitution saving throw. On a failed save,\
the creature takes 6 (1d12) Poison damage and has\
the Poisoned condition for 24 hours. On a successful\
save, the creature takes half as much damage only.\
