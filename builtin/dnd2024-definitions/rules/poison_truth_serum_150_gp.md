---
splat_schema: 1
splat_id: 49100522-211c-5116-959b-d6cbdc807828
splat_kind: rule
splat_key: dnd2024:rules/poison_truth_serum_150_gp
splat_name: Truth Serum (150 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison_truth_serum_150_gp
  parameters:
    poison:
      type: ingested
      cost_gp: 150
    save:
      ability: constitution
      dc: 11
    failure:
      condition:
        id: poisoned
        duration:
          value: 1
          unit: hour
      while_poisoned:
        knowingly_communicate_lie: false
  implementation: dnd2024:rules_poison_truth_serum_150_gp
splat_contracts:
- id: dnd2024:rules_poison_truth_serum_150_gp
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
  - dnd2024:rules_poison_truth_serum_150_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 00f6c060987596915c923aa628a215f3f56e4ff00c57f36f6dddc3883f4fc62b
  parent_name: Poison
  line_span:
    first: 9921
    count: 6
splat_clauses:
- key: source_rules
  text: '# Truth Serum (150 GP) Ingested Poison A creature subjected to Truth Serum must succeed on a
    DC 11 Constitution saving throw or have the Poisoned condition for 1 hour. The Poisoned creature can’t
    knowingly communicate a lie.'
  sha256: 36fed2ef1b3bafb95abdf1c49fbd5e6c15e7add6c8914937bb9eba8ac4c024fc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Truth Serum (150 GP)

<!-- SRD page 198 -->

Ingested Poison\
A creature subjected to Truth Serum must succeed\
on a DC 11 Constitution saving throw or have the\
Poisoned condition for 1 hour. The Poisoned crea-\
ture can’t knowingly communicate a lie.\
