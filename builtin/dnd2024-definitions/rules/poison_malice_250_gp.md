---
splat_schema: 1
splat_id: 33a773d0-815d-50c6-b92f-fff4b138a5ce
splat_kind: rule
splat_key: dnd2024:rules/poison_malice_250_gp
splat_name: Malice (250 GP)
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_malice_250_gp
  parameters:
    cost_gp: 250
    exposure_route: inhaled
    save:
      ability: constitution
      dc: 15
    failure:
      condition: poisoned
      duration:
        value: 1
        unit: hour
    while_poisoned_by_this:
      condition: blinded
  implementation: dnd2024:rules_poison_malice_250_gp
splat_contracts:
- id: dnd2024:rules_poison_malice_250_gp
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
  - conditions
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 197
  engine_gaps: &id001
  - dnd2024:rules_poison_malice_250_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: ec3072c753a33ea29eb3bd483d6deafa07a0d2fd6323c487de7596e4063df453
  parent_name: Poison
  line_span:
    first: 9859
    count: 6
splat_clauses:
- key: source_rules
  text: '# Malice (250 GP) Inhaled Poison A creature subjected to Malice must succeed on a DC 15 Constitution
    saving throw or have the Poisoned condition for 1 hour. The creature also has the Blinded condition
    while Poisoned in this way.'
  sha256: f4d5f803029d0be9aaa252d329eb2f9e33744933190c7b7f057b6b18fa85f4e7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Malice (250 GP)

<!-- SRD page 197 -->

Inhaled Poison\
A creature subjected to Malice must succeed on a\
DC 15 Constitution saving throw or have the Poi-\
soned condition for 1 hour. The creature also has\
the Blinded condition while Poisoned in this way.\
