---
splat_schema: 1
splat_id: c569bfa9-d21c-5911-a569-55d35d089f17
splat_kind: rule
splat_key: dnd2024:glossary/experience_points
splat_name: Experience Points
splat_origin:
  key: dnd2024:reference/glossary/experience_points
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/experience_points
  parameters:
    award:
      chooser: gm
      amount_type: integer
      triggers:
      - challenge_overcome
      - adventure_completed
    accumulation: add_to_total
    level_increase: level_advancement_threshold_reached
  implementation: dnd2024:glossary_experience_points
splat_contracts:
- id: dnd2024:glossary_experience_points
  inputs:
    subject:
      type: participant
      required: true
    xp_award:
      type: integer
      required: true
    advancement_table:
      type: definition
      required: true
  outputs:
  - xp_total
  - level
  events:
  - experience.awarded
  capabilities:
  - advancement
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 52d0f18d0fe9648e79512fa9cffc117b68cf7851b4325c9a8a15f2e544475f90
  source_path: builtin/srd-5.2.1/glossary/experience_points.md
  pages:
  - 182
  engine_gaps: &id001
  - dnd2024:glossary_experience_points
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: d563c27bf6a995937452dec2d8bf2812a7cb6a998a9d0eb707c32edabbd0f7a0
splat_clauses:
- key: source_rules
  text: '# Experience Points As they overcome challenges and complete adventures, characters earn Experience
    Points (XP), which are awarded by the Game Master. When a character’s XP total crosses certain thresholds,
    the character’s level increases. See also “Level Advancement.”'
  sha256: badd430b58ad80b565a3095635a80e276eb21c002b203a64cf39d7d34376514c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Experience Points

<!-- SRD page 182 -->

As they overcome challenges and complete ad-\
ventures, characters earn Experience Points (XP),\
which are awarded by the Game Master. When a\
character’s XP total crosses certain thresholds,\
the character’s level increases. See also “Level\
Advancement.”\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].
