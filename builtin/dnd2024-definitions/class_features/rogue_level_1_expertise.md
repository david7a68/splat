---
splat_schema: 1
splat_id: 9cc03f70-bcf5-54c3-a203-39a4bde2c3ac
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_1_expertise
splat_name: 'Level 1: Expertise'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_1_expertise
  parameters:
    expertise_choices:
    - rogue_level: 1
      count: 2
    - rogue_level: 6
      count: 2
    eligibility: own_skill_proficiencies
  implementation: dnd2024:class_features_rogue_level_1_expertise
splat_contracts:
- id: dnd2024:class_features_rogue_level_1_expertise
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
  outputs:
  - feature_state
  - choices
  - proficiencies
  events:
  - character.class_level_gained
  capabilities:
  - proficiencies
  - character_choices
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 61
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_1_expertise
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: cca91436b7ffef21f9748781727a4493255e3c4acf21ec918c24562ac544b06d
  parent_name: Rogue
  line_span:
    first: 5165
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 1: Expertise You gain Expertise in two of your skill proficiencies of your choice. Sleight
    of Hand and Stealth are recommended if you have proficiency in them. At Rogue level 6, you gain Expertise
    in two more of your skill proficiencies of your choice.'
  sha256: 577ff8e42ac7462b5c1d0a69abb4f8bc649b2b1577e8b7bf45ee522b5b339426
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Expertise

<!-- SRD page 61 -->

You gain Expertise in two of your skill proficiencies\
of your choice. Sleight of Hand and Stealth are rec-\
ommended if you have proficiency in them.\
At Rogue level 6, you gain Expertise in two more\
of your skill proficiencies of your choice.\
