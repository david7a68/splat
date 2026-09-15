---
splat_schema: 1
splat_id: 39d77ca4-da27-593c-ac77-25bcf215fc20
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_3_paladin_subclass
splat_name: 'Level 3: Paladin Subclass'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_3_paladin_subclass
  parameters:
    subclass_choice:
      count: 1
      class: paladin
      srd_options:
      - key: dnd2024:subclasses/paladin_paladin_subclass_oath_of_devotion
    grant_features:
      from: selected_subclass
      when_required_class_level_lte_current: true
      reevaluate_on_class_level_gain: true
  implementation: dnd2024:class_features_paladin_level_3_paladin_subclass
splat_contracts:
- id: dnd2024:class_features_paladin_level_3_paladin_subclass
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
  - 55
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_3_paladin_subclass
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: cf2783a15f307fca6995da7a9865c96a820edbd8569561b52897f58a7fd16159
  parent_name: Paladin
  line_span:
    first: 4594
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 3: Paladin Subclass You gain a Paladin subclass of your choice. The Oath of Devotion
    subclass is detailed after this class’s description. A subclass is a specialization that grants you
    features at certain Paladin levels. For the rest of your career, you gain each of your subclass’s
    features that are of your Paladin level or lower.'
  sha256: 9e79c96f2737fe0de167e3cf5c656f33310434b40b867d7026c1ffeedec02c53
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Paladin Subclass

<!-- SRD page 55 -->

You gain a Paladin subclass of your choice. The Oath\
of Devotion subclass is detailed after this class’s de-\
scription. A subclass is a specialization that grants\
you features at certain Paladin levels. For the rest\
of your career, you gain each of your subclass’s fea-\
tures that are of your Paladin level or lower.\
