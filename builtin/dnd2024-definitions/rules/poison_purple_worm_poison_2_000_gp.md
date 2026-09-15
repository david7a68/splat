---
splat_schema: 1
splat_id: 372f2450-098a-5935-9361-2d3cfa8f2f80
splat_kind: rule
splat_key: dnd2024:rules/poison_purple_worm_poison_2_000_gp
splat_name: Purple Worm Poison (2,000 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_purple_worm_poison_2_000_gp
  parameters:
    price_gp_per_dose: 2000
    poison_type: injury
    save:
      ability: constitution
      dc: 21
      failure:
        damage:
          dice:
            count: 10
            sides: 6
        average: 35
        type: poison
      success:
        damage_fraction: 0.5
        rounding: floor
  implementation: dnd2024:rule_poison_purple_worm_poison_2_000_gp
splat_contracts:
- id: dnd2024:rule_poison_purple_worm_poison_2_000_gp
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - effects
  events:
  - poison.exposure
  capabilities:
  - poisons
  - damage
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 198
  engine_gaps: &id001
  - dnd2024:rule_poison_purple_worm_poison_2_000_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: a1b113f94c836281ca451545aa7fa734ccd32fb8c50e050954f8f10f4d9c524e
  parent_name: Poison
  line_span:
    first: 9893
    count: 6
splat_clauses:
- key: source_rules
  text: '# Purple Worm Poison (2,000 GP) Injury Poison A creature subjected to Purple Worm Poison makes
    a DC 21 Constitution saving throw, taking 35 (10d6) Poison damage on a failed save or half as much
    damage on a successful one.'
  sha256: ce3ca61c0f9411db64989a356c01bfa0e51dd104c9fe2e79dae5738f434e052c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Purple Worm Poison (2,000 GP)

<!-- SRD page 198 -->

Injury Poison\
A creature subjected to Purple Worm Poison makes\
a DC 21 Constitution saving throw, taking 35 (10d6)\
Poison damage on a failed save or half as much dam-\
age on a successful one.\
