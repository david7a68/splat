---
splat_schema: 1
splat_id: 2133ca17-23c2-5d11-bfa7-3f6a5a89856a
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_2_expertise
splat_name: 'Level 2: Expertise'
splat_origin:
  key: dnd2024:reference/pages/032
splat_sources:
- title: SRD 5.2.1
  locator: pp. 32
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/bard_level_2_expertise
  parameters:
    expertise_grants:
    - bard_level: 2
      count: 2
      eligible:
        proficient: true
        expertise: false
      grant: expertise
    - bard_level: 9
      count: 2
      eligible:
        proficient: true
        expertise: false
      grant: expertise
    class_scope:
      class: bard
      minimum_level: 2
  implementation: dnd2024:class_features_bard_level_2_expertise
splat_contracts:
- id: dnd2024:class_features_bard_level_2_expertise
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
  - character.level_gained
  capabilities:
  - character_features
  - proficiencies
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 32
  engine_gaps: &id001
  - dnd2024:class_features_bard_level_2_expertise
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 01f808453da5860b4976ca300717f49b39e43c781c3d69e5eb17dde2695194ae
  parent_name: Bard
  line_span:
    first: 2647
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 2: Expertise You gain Expertise (see “Rules Glossary”) in two of your skill proficiencies
    of your choice. Performance and Persuasion are recommended if you have proficiency in them. At Bard
    level 9, you gain Expertise in two more of your skill proficiencies of your choice.'
  sha256: 7dbcbca2cab5ca885ac919e920064b034f0f6d1ee74b14d32e16d4ee8b328c2b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Expertise

<!-- SRD page 32 -->

You gain Expertise (see “Rules Glossary”) in two of\
your skill proficiencies of your choice. Performance\
and Persuasion are recommended if you have profi-\
ciency in them.\
At Bard level 9, you gain Expertise in two more of\
your skill proficiencies of your choice.\
