---
splat_schema: 1
splat_id: 61eb8bd2-0a43-51a9-a4d6-06a420afc613
splat_kind: rule
splat_key: dnd2024:glossary/skill
splat_name: Skill
splat_origin:
  key: dnd2024:reference/glossary/skill
splat_sources:
- title: SRD 5.2.1
  locator: pp. 188
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/skill
  parameters:
    context: ability_check
    skill_proficiency_adds_proficiency_bonus: true
    requires_check_associated_with_skill: true
  implementation: dnd2024:glossary_skill
splat_contracts:
- id: dnd2024:glossary_skill
  inputs:
    subject:
      type: participant
      required: true
    skill:
      type: definition
      required: true
    check:
      type: resolution
      required: true
  outputs:
  - check_bonus
  events:
  - ability_check.pending
  capabilities:
  - proficiency
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 15c0724e01d2205ff03f65dfc04496b4ceaf71c73bfd1212957216b49ffdad90
  source_path: builtin/srd-5.2.1/glossary/skill.md
  pages:
  - 188
  engine_gaps: &id001
  - dnd2024:glossary_skill
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 6bff81285653fe74fa60d0700d6efdcc4d761eb3de1a640c5a1b2939e3d73d98
splat_clauses:
- key: source_rules
  text: '# Skill A skill is an area of specialization associated with an ability check. If you have proficiency
    in a skill, you can add your Proficiency Bonus when you make an ability check associated with that
    skill. See also “Playing the Game” (“Proficiency”).'
  sha256: e4d5f76313b6c3ae097b08dd18bd750fa1a2530d8f90ebc8d022284f00185718
  status: contract_specified
splat_requires_mechanics: *id001
---

# Skill

<!-- SRD page 188 -->

A skill is an area of specialization associated with\
an ability check. If you have proficiency in a skill,\
you can add your Proficiency Bonus when you make\
an ability check associated with that skill. See also\
“Playing the Game” (“Proficiency”).\

Original layout: [[../../srd-5.2.1/pages/188|page 188]].
