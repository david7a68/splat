---
splat_schema: 1
splat_id: 4f4a42a4-7be8-5409-835f-5cbc741e0753
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_rest_and_recuperation
splat_name: Rest and Recuperation
splat_origin:
  key: dnd2024:reference/pages/194
splat_sources:
- title: SRD 5.2.1
  locator: pp. 194
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magical_contagions_rest_and_recuperation
  parameters:
    eligibility: infected_with_magical_contagion
    recuperation:
      days: 3
      forbid_long_rest_interrupting_activities: true
    end_save:
      ability: constitution
      dc: 15
    success:
      advantage_scope: saves_against_same_magical_contagion
      duration:
        value: 24
        unit: hour
  implementation: dnd2024:rules_magical_contagions_rest_and_recuperation
splat_contracts:
- id: dnd2024:rules_magical_contagions_rest_and_recuperation
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - recuperation.finished
  capabilities:
  - contagions
  - saving_throws
  - downtime
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 194
  engine_gaps: &id001
  - dnd2024:rules_magical_contagions_rest_and_recuperation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 55934680b9634514034cf464b5f20d7e1b0e5d5c11c67383efc5971211057baf
  parent_name: Magical Contagions
  line_span:
    first: 9513
    count: 9
splat_clauses:
- key: source_rules
  text: '# Rest and Recuperation If a creature infected with a magical contagion spends 3 days recuperating—engaging
    in no activities that would interrupt a Long Rest—the creature makes a DC 15 Constitution saving throw
    at the end of the recuperation period. On a successful save, the creature has Advantage on saving
    throws to fight off the magical contagion for the next 24 hours.'
  sha256: 766a56f112ae3a887ea056915b8215994d89c8d668292b9a064afea3b34d6e76
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rest and Recuperation

<!-- SRD page 194 -->

If a creature infected with a magical contagion\
spends 3 days recuperating—engaging in no\
activities that would interrupt a Long Rest—the\
creature makes a DC 15 Constitution saving throw\
at the end of the recuperation period. On a suc-\
cessful save, the creature has Advantage on saving\
throws to fight off the magical contagion for the\
next 24 hours.\
