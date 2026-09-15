---
splat_schema: 1
splat_id: 4d84c83d-12cd-5247-a158-4881b408170c
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_5_memorize_spell
splat_name: 'Level 5: Memorize Spell'
splat_origin:
  key: dnd2024:reference/pages/079
splat_sources:
- title: SRD 5.2.1
  locator: pp. 79
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/wizard_level_5_memorize_spell
  parameters:
    requires: study_own_spellbook
    replace_prepared_spells:
      maximum: 1
      feature: wizard_spellcasting
      minimum_spell_level: 1
      replacement_from: own_spellbook
      replacement_minimum_level: 1
  implementation: dnd2024:class_features_wizard_level_5_memorize_spell
splat_contracts:
- id: dnd2024:class_features_wizard_level_5_memorize_spell
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - rest.short_rest_completed
  capabilities:
  - spell_preparation
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 79
  engine_gaps: &id001
  - dnd2024:class_features_wizard_level_5_memorize_spell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: f5586b3e97ac972b4a7a2d5f19de0e79e3e3ff07a0cccf6297ab4dd24d3a058f
  parent_name: Wizard
  line_span:
    first: 6732
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 5: Memorize Spell Whenever you finish a Short Rest, you can study your spellbook and
    replace one of the level 1+ Wizard spells you have prepared for your Spellcasting feature with another
    level 1+ spell from the book.'
  sha256: 6885a34ca153b780a76eded4309363c59772aa37851cbd1af0ecc3a371417469
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Memorize Spell

<!-- SRD page 79 -->

Whenever you finish a Short Rest, you can study\
your spellbook and replace one of the level 1+ Wiz-\
ard spells you have prepared for your Spellcasting\
feature with another level 1+ spell from the book.\
