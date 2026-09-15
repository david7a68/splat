---
splat_schema: 1
splat_id: 05caec82-0065-5872-8cc1-05fc07d9addd
splat_kind: item
splat_key: dnd2024:items/spellguard_shield
splat_name: Spellguard Shield
splat_origin:
  key: dnd2024:reference/magic_item/spellguard_shield
splat_sources:
- title: SRD 5.2.1
  locator: pp. 244
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/spellguard_shield
  implementation: dnd2024:item_spellguard_shield
  parameters:
    equipment_state: held
    save_advantage_against:
    - spells
    - other_magical_effects
    attacks_against_user_disadvantage:
      attack_category: spell
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_armor
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_spellguard_shield
  inputs:
    actor:
      type: participant
      required: true
    source_item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choice:
      required: false
      type: choice
    event:
      required: false
      type: event
    campaign:
      required: false
      type: campaign
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - equipment.changed
  - saving_throw.pending
  - attack.roll_pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: a958ee4d6c5e0cb26c27e3d88e4ca777392c956427635a3760cf76df92fd9e08
  source_path: builtin/srd-5.2.1/magic_item/spellguard_shield.md
  pages:
  - 244
  engine_gaps: &id001
  - dnd2024:item_spellguard_shield
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 6a4a0dec8ce222269837dac8ee2a6a6f5bcb9221d44495c0de025a552e45e2cf
splat_clauses:
- key: source_rules
  text: '# Spellguard Shield Armor (Shield), Very Rare (Requires Attunement) While holding this Shield,
    you have Advantage on saving throws against spells and other magical effects, and spell attack rolls
    have Disadvantage against you.'
  sha256: ad187051bc0d58d2a3e017fd8e13a8408458cfee64503954f64610df8e61decc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spellguard Shield

<!-- SRD page 244 -->

Armor (Shield), Very Rare (Requires Attunement)\
While holding this Shield, you have Advantage on\
saving throws against spells and other magical\
effects, and spell attack rolls have Disadvantage\
against you.\

Original layout: [[../../srd-5.2.1/pages/244|page 244]].
