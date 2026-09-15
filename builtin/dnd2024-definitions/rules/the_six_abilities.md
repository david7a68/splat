---
splat_schema: 1
splat_id: cfa9ee0c-f55d-575e-bd99-cfca3340c430
splat_kind: rule
splat_key: dnd2024:rules/the_six_abilities
splat_name: The Six Abilities
splat_origin:
  key: dnd2024:reference/pages/005
splat_sources:
- title: SRD 5.2.1
  locator: pp. 5
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/the_six_abilities
  parameters:
    ability_ids:
    - strength
    - dexterity
    - constitution
    - intelligence
    - wisdom
    - charisma
    applies_to:
    - characters
    - monsters
  implementation: dnd2024:rules_the_six_abilities
splat_contracts:
- id: dnd2024:rules_the_six_abilities
  inputs:
    definition:
      type: definition
      required: true
  outputs:
  - typed_attributes
  events: []
  capabilities:
  - definition_catalogs
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 5
  engine_gaps: &id001
  - dnd2024:rules_the_six_abilities
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: ac963ded68b4eb7e847f6cfe1fd521539d23f616be0434d2bfaa457f0fb29099
  parent_name: The Six Abilities
  line_span:
    first: 54
    count: 12
splat_clauses:
- key: source_rules
  text: '# The Six Abilities All creatures—characters and monsters—have six abilities that measure physical
    and mental characteristics, as shown on the Ability Descriptions table. Ability Descriptions Ability
    Score Measures … Strength Physical might Dexterity Agility, reflexes, and balance Constitution Health
    and stamina Intelligence Reasoning and memory Wisdom Perceptiveness and mental fortitude Charisma
    Confidence, poise, and charm'
  sha256: 6b887df72b01861119519ad19339fb97ade374d08ffb7ef11805731552907700
  status: contract_specified
splat_requires_mechanics: *id001
---

# The Six Abilities

<!-- SRD page 5 -->

All creatures—characters and monsters—have six\
abilities that measure physical and mental charac-\
teristics, as shown on the Ability Descriptions table.\
Ability Descriptions\
Ability Score Measures …\
Strength Physical might\
Dexterity Agility, reflexes, and balance\
Constitution Health and stamina\
Intelligence Reasoning and memory\
Wisdom Perceptiveness and mental fortitude\
Charisma Confidence, poise, and charm\
