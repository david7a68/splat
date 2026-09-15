---
splat_schema: 1
splat_id: 924c1f25-428f-571d-a2af-14d189cc89fe
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_3_cleric_subclass
splat_name: 'Level 3: Cleric Subclass'
splat_origin:
  key: dnd2024:reference/pages/037
splat_sources:
- title: SRD 5.2.1
  locator: pp. 37
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/cleric_level_3_cleric_subclass
  parameters:
    subclass_choice:
      count: 1
      class: cleric
      srd_options:
      - key: dnd2024:subclasses/cleric_cleric_subclass_life_domain
    grant_features:
      from: selected_subclass
      when_required_class_level_lte_current: true
      reevaluate_on_class_level_gain: true
  implementation: dnd2024:class_features_cleric_level_3_cleric_subclass
splat_contracts:
- id: dnd2024:class_features_cleric_level_3_cleric_subclass
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
  - 37
  engine_gaps: &id001
  - dnd2024:class_features_cleric_level_3_cleric_subclass
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 3fce0a27f55f1a375d0184542b70aa2f89fb04f2b0ba4398a551c754adc046ce
  parent_name: Cleric
  line_span:
    first: 3090
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 3: Cleric Subclass You gain a Cleric subclass of your choice. The Life Domain subclass
    is detailed after this class’s description. A subclass is a specialization that grants you features
    at certain Cleric levels. For the rest of your career, you gain each of your subclass’s features that
    are of your Cleric level or lower.'
  sha256: 50d4c82af7f6755bb88d42c91eec6d1fdb86dbcd53cdb0c3b0f25ab03fabbabc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Cleric Subclass

<!-- SRD page 37 -->

You gain a Cleric subclass of your choice. The Life\
Domain subclass is detailed after this class’s de-\
scription. A subclass is a specialization that grants\
you features at certain Cleric levels. For the rest of\
your career, you gain each of your subclass’s fea-\
tures that are of your Cleric level or lower.\
