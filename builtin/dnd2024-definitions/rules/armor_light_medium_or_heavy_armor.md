---
splat_schema: 1
splat_id: e4fcdad4-0fdb-52b8-a649-c02b37c242f8
splat_kind: rule
splat_key: dnd2024:rules/armor_light_medium_or_heavy_armor
splat_name: Light, Medium, or Heavy Armor
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/armor_light_medium_or_heavy_armor
  parameters:
    when:
      wearing_armor_category:
      - light
      - medium
      - heavy
      trained_in_worn_armor: false
    effects:
      d20_tests_using:
      - strength
      - dexterity
      roll_mode: disadvantage
      spellcasting_allowed: false
  implementation: dnd2024:rule_armor_light_medium_or_heavy_armor
splat_contracts:
- id: dnd2024:rule_armor_light_medium_or_heavy_armor
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - item_effects
  events: []
  capabilities:
  - equipment
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 92
  engine_gaps: &id001
  - dnd2024:rule_armor_light_medium_or_heavy_armor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 48ad1eeccdd3c1bb94412b5ba3d2cd501d5c6bcac2aad8b2f7be7210475454cc
  parent_name: Armor
  line_span:
    first: 7952
    count: 5
splat_clauses:
- key: source_rules
  text: '# Light, Medium, or Heavy Armor If you wear Light, Medium, or Heavy armor and lack training with
    it, you have Disadvantage on any D20 Test that involves Strength or Dexterity, and you can’t cast
    spells.'
  sha256: a8fa78c33cb17fce3dd4a69a32deefcf6b953b2ac07d371149b757c6c027d8a9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Light, Medium, or Heavy Armor

<!-- SRD page 92 -->

If you wear Light, Medium, or Heavy armor and lack\
training with it, you have Disadvantage on any D20\
Test that involves Strength or Dexterity, and you\
can’t cast spells.\
