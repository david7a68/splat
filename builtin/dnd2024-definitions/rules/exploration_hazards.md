---
splat_schema: 1
splat_id: 87d335dc-50bc-52a8-a0d9-3ae5bbb41147
splat_kind: rule
splat_key: dnd2024:rules/exploration_hazards
splat_name: Hazards
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/exploration_hazards
  parameters:
    hazard_definitions:
    - key: dnd2024:glossary/burning_hazard
    - key: dnd2024:glossary/falling_hazard
    - key: dnd2024:glossary/suffocation_hazard
    - key: dnd2024:glossary/dehydration_hazard
    - key: dnd2024:glossary/malnutrition_hazard
  implementation: dnd2024:rules_exploration_hazards
splat_contracts:
- id: dnd2024:rules_exploration_hazards
  inputs: {}
  outputs:
  - hazard_definitions
  events: []
  capabilities:
  - definition_catalogs
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 12
  engine_gaps: &id001
  - dnd2024:rules_exploration_hazards
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: c02623f7b0f8641d264b51414d99ce54fcf0545bae452fe2eb47b15a4715a3b1
  parent_name: Exploration
  line_span:
    first: 712
    count: 6
splat_clauses:
- key: source_rules
  text: '# Hazards Monsters are the main perils characters face, but other dangers await. “Rules Glossary”
    defines the following hazards: Burning Falling Suffocation Dehydration Malnutrition'
  sha256: e79987030b5360ac57afcf2cba621d8d8224d3d051166f16215bb44d52127939
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hazards

<!-- SRD page 12 -->

Monsters are the main perils characters face, but\
other dangers await. “Rules Glossary” defines the\
following hazards:\
Burning Falling Suffocation\
Dehydration Malnutrition\
