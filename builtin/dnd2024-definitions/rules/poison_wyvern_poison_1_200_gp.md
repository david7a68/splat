---
splat_schema: 1
splat_id: a7ccef37-0c61-5edd-9b61-3852277811a9
splat_kind: rule
splat_key: dnd2024:rules/poison_wyvern_poison_1_200_gp
splat_name: Wyvern Poison (1,200 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_wyvern_poison_1_200_gp
  parameters:
    cost_gp: 1200
    exposure_route: injury
    save:
      ability: constitution
      dc: 14
    damage:
      dice:
        count: 7
        sides: 6
      average: 24
      type: poison
    success_damage: half_round_down
  implementation: dnd2024:rules_poison_wyvern_poison_1_200_gp
splat_contracts:
- id: dnd2024:rules_poison_wyvern_poison_1_200_gp
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - poison.exposure
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
  - 198
  engine_gaps: &id001
  - dnd2024:rules_poison_wyvern_poison_1_200_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 479b95876ed22a4efbb46bc6a50834b218a783cd490d8974602f8d7e11da2be8
  parent_name: Poison
  line_span:
    first: 9927
    count: 6
splat_clauses:
- key: source_rules
  text: '# Wyvern Poison (1,200 GP) Injury Poison A creature subjected to Wyvern Poison makes a DC 14
    Constitution saving throw, taking 24 (7d6) Poison damage on a failed save or half as much damage on
    a successful one.'
  sha256: 9caf410270693ff7e8cd94de81c24f8e69c60417a6d51f39f0b31a04ca33f37d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wyvern Poison (1,200 GP)

<!-- SRD page 198 -->

Injury Poison\
A creature subjected to Wyvern Poison makes a DC\
14 Constitution saving throw, taking 24 (7d6) Poi-\
son damage on a failed save or half as much damage\
on a successful one.\
