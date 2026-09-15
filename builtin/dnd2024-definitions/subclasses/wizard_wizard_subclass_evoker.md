---
splat_schema: 1
splat_id: 74c1cc78-2a18-533e-bc9f-7f2d3b211559
splat_kind: rule_group
splat_key: dnd2024:subclasses/wizard_wizard_subclass_evoker
splat_name: 'Wizard Subclass: Evoker'
splat_origin:
  key: dnd2024:reference/pages/082
splat_sources:
- title: SRD 5.2.1
  locator: pp. 82
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: subclass
  members:
  - key: dnd2024:class_features/evoker_level_10_empowered_evocation
  - key: dnd2024:class_features/evoker_level_14_overchannel
  - key: dnd2024:class_features/evoker_level_3_evocation_savant
  - key: dnd2024:class_features/evoker_level_3_potent_cantrip
  - key: dnd2024:class_features/evoker_level_6_sculpt_spells
splat_rule:
  mechanic: dnd2024:subclasses/wizard_wizard_subclass_evoker
  implementation: dnd2024:subclasses_wizard_wizard_subclass_evoker
  parameters:
    class: wizard
    subclass: evoker
    feature_grants:
    - feature:
        key: dnd2024:class_features/evoker_level_10_empowered_evocation
      minimum_class_level: 10
    - feature:
        key: dnd2024:class_features/evoker_level_14_overchannel
      minimum_class_level: 14
    - feature:
        key: dnd2024:class_features/evoker_level_3_evocation_savant
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/evoker_level_3_potent_cantrip
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/evoker_level_6_sculpt_spells
      minimum_class_level: 6
splat_contracts:
- id: dnd2024:subclasses_wizard_wizard_subclass_evoker
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - character.subclass_selected
  - character.class_level_gained
  capabilities:
  - class_progression
splat_conversion:
  status: specified
  category: subclass
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 82
  engine_gaps: &id001
  - dnd2024:subclasses_wizard_wizard_subclass_evoker
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 324b70116601e320ed5db6c1686bff114fbf6f1f0eabf854702193a3368a58ec
  parent_name: Wizard
  line_span:
    first: 7007
    count: 9
splat_clauses:
- key: source_rules
  text: '# Wizard Subclass: Evoker Create Explosive Elemental Effects Your studies focus on magic that
    creates powerful elemental effects such as bitter cold, searing flame, rolling thunder, crackling
    lightning, and burning acid. Some Evokers find employment in military forces, serving as artillery
    to blast armies from afar. Others use their power to protect others, while some seek their own gain.'
  sha256: a504349c5b4c2ae3d50753c3b2f5331ba609f3f2329636344fb8388702eec176
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wizard Subclass: Evoker

<!-- SRD page 82 -->

Create Explosive Elemental Effects\
Your studies focus on magic that creates powerful\
elemental effects such as bitter cold, searing flame,\
rolling thunder, crackling lightning, and burning\
acid. Some Evokers find employment in military\
forces, serving as artillery to blast armies from\
afar. Others use their power to protect others, while\
some seek their own gain.\
