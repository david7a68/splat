---
splat_schema: 1
splat_id: 1459cd0d-5d5e-5fc9-bdcc-531985823a54
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_class_features
splat_name: Class Features
splat_origin:
  key: dnd2024:reference/pages/025
splat_sources:
- title: SRD 5.2.1
  locator: pp. 25
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/multiclassing_class_features
  parameters:
    gain_features_for: new_class_level
    apply_class_multiclass_rules: true
    special_handling:
    - extra_attack
    - spellcasting
    - alternative_ac_calculations
    special_handling_rules:
    - key: dnd2024:rules/multiclassing_extra_attack
    - key: dnd2024:rules/multiclassing_spellcasting
    - key: dnd2024:rules/multiclassing_armor_class
  implementation: dnd2024:rules_multiclassing_class_features
splat_contracts:
- id: dnd2024:rules_multiclassing_class_features
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - character.level_gained
  capabilities:
  - multiclassing
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 25
  engine_gaps: &id001
  - dnd2024:rules_multiclassing_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 725eed764b07f4585f604dc43b2091fa1860d0b32f2232a5e51c66237e2c8df5
  parent_name: Multiclassing
  line_span:
    first: 1951
    count: 9
splat_clauses:
- key: source_rules
  text: '# Class Features When you gain a new level in a class, you get its features for that level. A
    few features have additional rules when you’re multiclassing. Check the information about multiclassing
    included in each of your classes’ descriptions. Special rules apply to Extra Attack, Spellcasting,
    and features (such as Unarmored Defense) that give you alternative ways to calculate your Armor Class.'
  sha256: b3ee24ca06901029d0328a1f4c1285ba0b66f92b7db29f74cda5c325b81429d9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Class Features

<!-- SRD page 25 -->

When you gain a new level in a class, you get its fea-\
tures for that level. A few features have additional\
rules when you’re multiclassing. Check the infor-\
mation about multiclassing included in each of your\
classes’ descriptions.\
Special rules apply to Extra Attack, Spellcasting,\
and features (such as Unarmored Defense) that give\
you alternative ways to calculate your Armor Class.\
