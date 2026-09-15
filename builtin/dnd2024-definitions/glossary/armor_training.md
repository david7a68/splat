---
splat_schema: 1
splat_id: 0e90ccf1-865b-5900-81dc-72f525b2b1b8
splat_kind: rule
splat_key: dnd2024:glossary/armor_training
splat_name: Armor Training
splat_origin:
  key: dnd2024:reference/glossary/armor_training
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/armor_training
  parameters:
    categories:
    - light
    - medium
    - heavy
    - shield
    untrained_armor:
      categories:
      - light
      - medium
      - heavy
      d20_disadvantage_abilities:
      - strength
      - dexterity
      spellcasting_allowed: false
    untrained_shield:
      shield_ac_bonus: 0
  implementation: dnd2024:glossary_armor_training
splat_contracts:
- id: dnd2024:glossary_armor_training
  inputs:
    subject:
      type: participant
      required: true
    armor:
      type: item_instance
      required: true
  outputs:
  - drawbacks
  events:
  - equipment.changed
  - d20_test.pending
  - spell.cast_pending
  capabilities:
  - equipment
  - checks
  - spellcasting
splat_conversion:
  status: specified
  category: glossary
  source_sha256: e1d2a9de1e1ef63e0ffb6892deefeec73b033ea6c0d6f0d0f375cfd4cf1adc19
  source_path: builtin/srd-5.2.1/glossary/armor_training.md
  pages:
  - 177
  engine_gaps: &id001
  - dnd2024:glossary_armor_training
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 0e0d14fd8b265b8fdef966285155d4e81308ff67ec3c1e2a1ff59256521033e4
splat_clauses:
- key: source_rules
  text: '# Armor Training Armor training allows you to use armor of a certain category without the following
    drawbacks. If you wear Light, Medium, or Heavy armor and lack training with it, you have Disadvantage
    on any D20 Test that involves Strength or Dexterity, and you can’t cast spells. If you use a Shield
    and lack training with it, you don’t gain its AC bonus. See also “Disadvantage” and “Equipment” (“Armor”).'
  sha256: 28061d88efae76bdff99e5f73188011cb6055627bf0befe739f2d32bfa95d66c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Armor Training

<!-- SRD page 177 -->

Armor training allows you to use armor of a certain\
category without the following drawbacks. If you\
wear Light, Medium, or Heavy armor and lack train-\
ing with it, you have Disadvantage on any D20 Test\
that involves Strength or Dexterity, and you can’t\
cast spells. If you use a Shield and lack training with\
it, you don’t gain its AC bonus. See also “Disadvan-\
tage” and “Equipment” (“Armor”).\

Original layout: [[../../srd-5.2.1/pages/177|page 177]].
