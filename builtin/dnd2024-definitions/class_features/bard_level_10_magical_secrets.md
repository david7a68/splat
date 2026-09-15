---
splat_schema: 1
splat_id: e7c656be-c514-5e20-b917-3ba1d417f0f5
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_10_magical_secrets
splat_name: 'Level 10: Magical Secrets'
splat_origin:
  key: dnd2024:reference/pages/033
splat_sources:
- title: SRD 5.2.1
  locator: pp. 33
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/bard_level_10_magical_secrets
  parameters:
    spell_list_expansion:
    - bard
    - cleric
    - druid
    - wizard
    applies_to:
      new_preparations:
        event: bard.level_gained
        minimum_level: 10
        prepared_count_increased: true
      replacements: true
    counts_as_class: bard
    other_preparation_constraints_unchanged: true
    class_scope:
      class: bard
      minimum_level: 10
  implementation: dnd2024:class_features_bard_level_10_magical_secrets
splat_contracts:
- id: dnd2024:class_features_bard_level_10_magical_secrets
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - character.level_gained
  - spell.preparation_changed
  capabilities:
  - character_features
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 33
  engine_gaps: &id001
  - dnd2024:class_features_bard_level_10_magical_secrets
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 4d30098f55d3208449a97998d7e261e9a81c9dd4d81a5a7e74e0df712fc6edcd
  parent_name: Bard
  line_span:
    first: 2687
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 10: Magical Secrets You’ve learned secrets from various magical traditions. Whenever
    you reach a Bard level (including this level) and the Prepared Spells number in the Bard Features
    table increases, you can choose any of your new prepared spells from the Bard, Cleric, Druid, and
    Wizard spell lists, and the chosen spells count as Bard spells for you (see a class’s section for
    its spell list). In addition, whenever you replace a spell prepared for this class, you can replace
    it with a spell from those lists.'
  sha256: a191b2eb0440fb9a4bbea7d46e19c046f4ac5c8df9d2997bd1c7b809162cad0c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 10: Magical Secrets

<!-- SRD page 33 -->

You’ve learned secrets from various magical tradi-\
tions. Whenever you reach a Bard level (including\
this level) and the Prepared Spells number in the\
Bard Features table increases, you can choose any\
of your new prepared spells from the Bard, Cleric,\
Druid, and Wizard spell lists, and the chosen spells\
count as Bard spells for you (see a class’s section for\
its spell list). In addition, whenever you replace a\
spell prepared for this class, you can replace it with\
a spell from those lists.\
