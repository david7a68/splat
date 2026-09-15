---
splat_schema: 1
splat_id: b8aaf636-c9ef-547d-b777-d2c8ab19ffaa
splat_kind: rule
splat_key: dnd2024:glossary/ability_score_and_modifier
splat_name: Ability Score and Modifier
splat_origin:
  key: dnd2024:reference/glossary/ability_score_and_modifier
splat_sources:
- title: SRD 5.2.1
  locator: pp. 176
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/ability_score_and_modifier
  parameters:
    abilities:
    - strength
    - dexterity
    - constitution
    - intelligence
    - wisdom
    - charisma
    d20_modifier:
      when: corresponding_ability_used
      add:
        var: ability_modifier
    other_modifier_application: when_rule_requires
  implementation: dnd2024:glossary_ability_score_and_modifier
splat_contracts:
- id: dnd2024:glossary_ability_score_and_modifier
  inputs:
    subject:
      type: participant
      required: true
    ability:
      type: choice
      required: true
    ability_modifier:
      type: integer
      required: true
  outputs:
  - modifier
  events: []
  capabilities:
  - checks
  - statistics
splat_conversion:
  status: specified
  category: glossary
  source_sha256: bde09263ce18736e963e937802f1cf5308929aa876791d8aff5218661b37dcce
  source_path: builtin/srd-5.2.1/glossary/ability_score_and_modifier.md
  pages:
  - 176
  engine_gaps: &id001
  - dnd2024:glossary_ability_score_and_modifier
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: dfc7d096aa3a3496d1bfff0f16c0f32080c8b7c5ef65e3265fed59327b9247c3
splat_clauses:
- key: source_rules
  text: '# Ability Score and Modifier A creature has six ability scores—Strength, Dexterity, Constitution,
    Intelligence, Wisdom, and Charisma—each of which has a corresponding modifier. Add the modifier when
    you make a D20 Test with the corresponding ability or when a rule asks you to do so. See also “Playing
    the Game” (“The Six Abilities”).'
  sha256: 3f52ce41c6f4742d2397109fb26a9d9b6de7cf8a4a36e0358c7dcda5be9729e6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ability Score and Modifier

<!-- SRD page 176 -->

A creature has six ability scores—Strength, Dex-\
terity, Constitution, Intelligence, Wisdom, and\
Charisma—each of which has a corresponding mod-\
ifier. Add the modifier when you make a D20 Test\
with the corresponding ability or when a rule asks\
you to do so. See also “Playing the Game” (“The Six\
Abilities”).\

Original layout: [[../../srd-5.2.1/pages/176|page 176]].
