---
splat_schema: 1
splat_id: f2af8a33-f692-53f2-b46a-7ff1e3a4fe3f
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_3_barbarian_subclass
splat_name: 'Level 3: Barbarian Subclass'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_3_barbarian_subclass
  parameters:
    subclass_choice:
      count: 1
      class: barbarian
      srd_options:
      - key: dnd2024:subclasses/barbarian_path_of_the_berserker
    grant_features:
      from: selected_subclass
      when_required_class_level_lte_current: true
      reevaluate_on_class_level_gain: true
  implementation: dnd2024:class_features_barbarian_level_3_barbarian_subclass
splat_contracts:
- id: dnd2024:class_features_barbarian_level_3_barbarian_subclass
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
  - 29
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_3_barbarian_subclass
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 8d6faada2c8d5ecd8387490e15f69f2d30671133988ae8a32946f8f9695bbb46
  parent_name: Barbarian
  line_span:
    first: 2349
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 3: Barbarian Subclass You gain a Barbarian subclass of your choice. The Path of the Berserker
    subclass is detailed after this class’s description. A subclass is a specialization that grants you
    features at certain Barbarian levels. For the rest of your career, you gain each of your subclass’s
    features that are of your Barbarian level or lower.'
  sha256: cac94e01cfa37af49bba3a2100889dd752a30a588b23c67aa29b837c42acb175
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Barbarian Subclass

<!-- SRD page 29 -->

You gain a Barbarian subclass of your choice. The\
Path of the Berserker subclass is detailed after this\
class’s description. A subclass is a specialization\
that grants you features at certain Barbarian levels.\
For the rest of your career, you gain each of your\
subclass’s features that are of your Barbarian level\
or lower.\
