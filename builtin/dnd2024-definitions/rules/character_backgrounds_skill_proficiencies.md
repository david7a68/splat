---
splat_schema: 1
splat_id: de89f23a-b9fe-52d8-9655-8050444b3228
splat_kind: rule
splat_key: dnd2024:rules/character_backgrounds_skill_proficiencies
splat_name: Skill Proficiencies
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/character_backgrounds_skill_proficiencies
  parameters:
    background_skill_grants:
      count: 2
      from: background_specified_skills
  implementation: dnd2024:rules_character_backgrounds_skill_proficiencies
splat_contracts:
- id: dnd2024:rules_character_backgrounds_skill_proficiencies
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - character_updates
  events: []
  capabilities:
  - character_creation
  - progression
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 83
  engine_gaps: &id001
  - dnd2024:rules_character_backgrounds_skill_proficiencies
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: d49ffcfe4f788a2746fee55907940173bbd977653b93354a192e683f7f6bea0b
  parent_name: Character Backgrounds
  line_span:
    first: 7075
    count: 3
splat_clauses:
- key: source_rules
  text: '# Skill Proficiencies A background gives your character proficiency in two specified skills.'
  sha256: 9ed459e02865827c95e167f42239edf710b347815afaead6dbd701afde06af8f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Skill Proficiencies

<!-- SRD page 83 -->

A background gives your character proficiency in\
two specified skills.\
