---
splat_schema: 1
splat_id: e015a36b-6bda-52b0-81e1-60e659459416
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_2_scholar
splat_name: 'Level 2: Scholar'
splat_origin:
  key: dnd2024:reference/pages/078
splat_sources:
- title: SRD 5.2.1
  locator: pp. 78
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/wizard_level_2_scholar
  parameters:
    expertise:
      count: 1
      eligible:
        proficient: true
        expertise: false
        skills:
        - arcana
        - history
        - investigation
        - medicine
        - nature
        - religion
      grant: expertise
    class_scope:
      class: wizard
      minimum_level: 2
  implementation: dnd2024:class_features_wizard_level_2_scholar
splat_contracts:
- id: dnd2024:class_features_wizard_level_2_scholar
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - character.feature_acquired
  capabilities:
  - character_features
  - proficiencies
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 78
  engine_gaps: &id001
  - dnd2024:class_features_wizard_level_2_scholar
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 5d284726227029805bb09cf5601be21b5a66ad4e9b056f44f571de4dd77ecd5b
  parent_name: Wizard
  line_span:
    first: 6688
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 2: Scholar While studying magic, you also specialized in another field of study. Choose
    one of the following skills in which you have proficiency: Arcana, History, Investigation, Medicine,
    Nature, or Religion. You have Expertise in the chosen skill.'
  sha256: 8d5a6a81709b7e26b33b1916bac65ca0dda5729bb282c5efa7eb4503da5c2c90
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Scholar

<!-- SRD page 78 -->

While studying magic, you also specialized in an-\
other field of study. Choose one of the following\
skills in which you have proficiency: Arcana, His-\
tory, Investigation, Medicine, Nature, or Religion.\
You have Expertise in the chosen skill.\
