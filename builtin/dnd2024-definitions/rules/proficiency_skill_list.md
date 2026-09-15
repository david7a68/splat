---
splat_schema: 1
splat_id: 24b9af6d-f0f1-58b4-bfe4-6bb4a20ad639
splat_kind: rule
splat_key: dnd2024:rules/proficiency_skill_list
splat_name: Skill List
splat_origin:
  key: dnd2024:reference/pages/009
splat_sources:
- title: SRD 5.2.1
  locator: pp. 9
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/proficiency_skill_list
  parameters:
    default_skill_abilities:
      acrobatics: dexterity
      animal_handling: wisdom
      arcana: intelligence
      athletics: strength
      deception: charisma
      history: intelligence
      insight: wisdom
      intimidation: charisma
      investigation: intelligence
      medicine: wisdom
      nature: intelligence
      perception: wisdom
      performance: charisma
      persuasion: charisma
      religion: intelligence
      sleight_of_hand: dexterity
      stealth: dexterity
      survival: wisdom
    ability_assignment: usual_not_exclusive
  implementation: dnd2024:rules_proficiency_skill_list
splat_contracts:
- id: dnd2024:rules_proficiency_skill_list
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
  - 9
  engine_gaps: &id001
  - dnd2024:rules_proficiency_skill_list
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: c75e9ebf4dd82cfaf9ff539cc12a2507da79c1ef9b45c6af72d94f83fa25b78e
  parent_name: Proficiency
  line_span:
    first: 415
    count: 4
splat_clauses:
- key: source_rules
  text: '# Skill List The skills are shown on the Skills table, which notes example uses for each skill
    proficiency as well as the ability check the skill most often applies to.'
  sha256: 8407197fa31bb6d45d42126445a35f4c179d4975a69a57f83481190140f23bae
  status: contract_specified
splat_requires_mechanics: *id001
---

# Skill List

<!-- SRD page 9 -->

The skills are shown on the Skills table, which notes\
example uses for each skill proficiency as well as\
the ability check the skill most often applies to.\
