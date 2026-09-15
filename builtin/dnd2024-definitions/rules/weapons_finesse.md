---
splat_schema: 1
splat_id: 3f08a1f0-8f5c-5587-bec0-266176f3de06
splat_kind: rule
splat_key: dnd2024:rules/weapons_finesse
splat_name: Finesse
splat_origin:
  key: dnd2024:reference/pages/089
splat_sources:
- title: SRD 5.2.1
  locator: pp. 89
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_finesse
  parameters:
    ability_choice:
    - strength
    - dexterity
    same_ability_for_attack_and_damage: true
  implementation: dnd2024:rules_weapons_finesse
splat_contracts:
- id: dnd2024:rules_weapons_finesse
  inputs:
    actor:
      type: participant
      required: true
    test:
      type: resolution
      required: true
  outputs:
  - test_modifiers
  - test_result
  events: []
  capabilities:
  - d20_tests
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 89
  engine_gaps: &id001
  - dnd2024:rules_weapons_finesse
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 7363e6340934bd2cbb0688ef094c2bd296279197b0df5e8df202e334d34238e8
  parent_name: Weapons
  line_span:
    first: 7706
    count: 5
splat_clauses:
- key: source_rules
  text: '# Finesse When making an attack with a Finesse weapon, use your choice of your Strength or Dexterity
    modifier for the attack and damage rolls. You must use the same modifier for both rolls.'
  sha256: ce1ec65eaf60214988c1e01bab09a80ee83002f5d85c5b841574cbb3ba140f8e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Finesse

<!-- SRD page 89 -->

When making an attack with a Finesse weapon, use\
your choice of your Strength or Dexterity modifier\
for the attack and damage rolls. You must use the\
same modifier for both rolls.\
