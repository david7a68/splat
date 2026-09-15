---
splat_schema: 1
splat_id: cf3565a4-1d71-5569-b07d-9bc6b80a4329
splat_kind: rule
splat_key: dnd2024:rules/creating_a_background_3_choose_skill_proficiencies
splat_name: '3: Choose Skill Proficiencies'
splat_origin:
  key: dnd2024:reference/pages/193
splat_sources:
- title: SRD 5.2.1
  locator: pp. 193
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/creating_a_background_3_choose_skill_proficiencies
  parameters:
    skill_proficiencies:
      count: 2
      selection: background_designer_choice
      appropriate_to_background: true
    require_related_background_ability: false
  implementation: dnd2024:rules_creating_a_background_3_choose_skill_proficiencies
splat_contracts:
- id: dnd2024:rules_creating_a_background_3_choose_skill_proficiencies
  inputs:
    background:
      type: definition
      required: true
    skills:
      type: choice
      required: true
  outputs:
  - skill_proficiencies
  events: []
  capabilities:
  - character_options
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 193
  engine_gaps: &id001
  - dnd2024:rules_creating_a_background_3_choose_skill_proficiencies
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: d77efca5a62f35204e554bb8e71c753ba745207b5634f01ca0dc48757b07cae7
  parent_name: Creating a Background
  line_span:
    first: 9419
    count: 5
splat_clauses:
- key: source_rules
  text: '# 3: Choose Skill Proficiencies Choose two skills appropriate for the background. There needn’t
    be a relationship between the skill proficiencies a background grants and the ability scores it increases.'
  sha256: 330f7a4891c427f14b1931634863f7685d7cab841255a16a75fd2404f8d42a66
  status: contract_specified
splat_requires_mechanics: *id001
---

# 3: Choose Skill Proficiencies

<!-- SRD page 193 -->

Choose two skills appropriate for the background.\
There needn’t be a relationship between the skill\
proficiencies a background grants and the ability\
scores it increases.\
