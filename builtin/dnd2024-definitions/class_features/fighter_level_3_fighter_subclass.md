---
splat_schema: 1
splat_id: 0711c1c0-69f4-58aa-af31-bcc55926bdf2
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_3_fighter_subclass
splat_name: 'Level 3: Fighter Subclass'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_3_fighter_subclass
  parameters:
    subclass_choice:
      count: 1
      class: fighter
      srd_options:
      - key: dnd2024:subclasses/fighter_fighter_subclass_champion
    grant_features:
      from: selected_subclass
      when_required_class_level_lte_current: true
      reevaluate_on_class_level_gain: true
  implementation: dnd2024:class_features_fighter_level_3_fighter_subclass
splat_contracts:
- id: dnd2024:class_features_fighter_level_3_fighter_subclass
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
  events:
  - character.feature_gained
  - character.level_gained
  capabilities:
  - character_choices
  - class_progression
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_3_fighter_subclass
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 06cff5cf34c290aa227715011e6dcf9890101e20aae9b5a3bd85744021d5d2b4
  parent_name: Fighter
  line_span:
    first: 4001
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 3: Fighter Subclass You gain a Fighter subclass of your choice. The Champion subclass
    is detailed after this class’s description. A subclass is a specialization that grants you features
    at certain Fighter levels. For the rest of your career, you gain each of your subclass’s features
    that are of your Fighter level or lower.'
  sha256: c614968f8d3c84887919f7698e7be3fa782e9e2f93ea4a5061d694c31458e62b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Fighter Subclass

<!-- SRD page 48 -->

You gain a Fighter subclass of your choice. The\
Champion subclass is detailed after this class’s de-\
scription. A subclass is a specialization that grants\
you features at certain Fighter levels. For the rest\
of your career, you gain each of your subclass’s fea-\
tures that are of your Fighter level or lower.\
