---
splat_schema: 1
splat_id: 3ddfd7cf-91ff-5401-a196-606f53747af4
splat_kind: rule
splat_key: dnd2024:rules/poison_spider_s_sting_200_gp
splat_name: Spider’s Sting (200 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_spider_s_sting_200_gp
  parameters:
    cost_gp: 200
    exposure_route: injury
    save:
      ability: constitution
      dc: 13
    failure:
      condition: poisoned
      duration:
        value: 1
        unit: hour
    failure_margin_at_least_5:
      condition: unconscious
      duration: while_poisoned_by_this
    unconscious_ends_early:
    - damage_taken
    - other_creature_action_shake_awake
  implementation: dnd2024:rules_poison_spider_s_sting_200_gp
splat_contracts:
- id: dnd2024:rules_poison_spider_s_sting_200_gp
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - poison.exposure
  - damage.applied
  - action.shake_awake
  capabilities:
  - poison
  - conditions
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 198
  engine_gaps: &id001
  - dnd2024:rules_poison_spider_s_sting_200_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 6c474d1dd576e3a9ef2d67a4ae078eace7c82e4b4fe88a5fb82fbd643003e4b2
  parent_name: Poison
  line_span:
    first: 9905
    count: 9
splat_clauses:
- key: source_rules
  text: '# Spider’s Sting (200 GP) Injury Poison A creature subjected to Spider’s Sting must succeed on
    a DC 13 Constitution saving throw or have the Poisoned condition for 1 hour. If the creature fails
    the save by 5 or more, the creature also has the Unconscious condition while Poisoned in this way.
    The creature wakes up if it takes damage or if another creature takes an action to shake it awake.'
  sha256: 6b71089bcb1214741c66b18188e612d375c06c4cca1d5f72a1910a9f8b3e445a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spider’s Sting (200 GP)

<!-- SRD page 198 -->

Injury Poison\
A creature subjected to Spider’s Sting must succeed\
on a DC 13 Constitution saving throw or have the\
Poisoned condition for 1 hour. If the creature fails\
the save by 5 or more, the creature also has the Un-\
conscious condition while Poisoned in this way. The\
creature wakes up if it takes damage or if another\
creature takes an action to shake it awake.\
