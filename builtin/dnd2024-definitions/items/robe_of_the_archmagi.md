---
splat_schema: 1
splat_id: f07a831d-667a-58d5-892d-a1f6f2ff5157
splat_kind: item
splat_key: dnd2024:items/robe_of_the_archmagi
splat_name: Robe of the Archmagi
splat_origin:
  key: dnd2024:reference/magic_item/robe_of_the_archmagi
splat_sources:
- title: SRD 5.2.1
  locator: pp. 240
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: required
    attunement_requirement_text: a Sorcerer, Warlock, or Wizard
splat_rule:
  mechanic: dnd2024:items/robe_of_the_archmagi
  implementation: dnd2024:item_robe_of_the_archmagi
  parameters:
    equipped: worn
    attunement_classes:
    - sorcerer
    - warlock
    - wizard
    armor_class:
      requires_no_armor: true
      base:
        add:
        - 15
        - dexterity_modifier
    saving_throw_advantage:
      against:
      - spells
      - other_magical_effects
    spell_save_dc_bonus: 2
    spell_attack_bonus: 2
    attunement_required: true
splat_contracts:
- id: dnd2024:item_robe_of_the_archmagi
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - item.equipment_changed
  - saving_throw.pending
  - spell.cast_pending
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 07a9eea89ac92ef2019fce9cb4b561d2d011fd82a93ca94adfb993ecaac628aa
  source_path: builtin/srd-5.2.1/magic_item/robe_of_the_archmagi.md
  pages:
  - 240
  engine_gaps: &id001
  - dnd2024:item_robe_of_the_archmagi
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 63cca6d110fa9bfa4c75d8a749f931d6b451be2575bc29ce6da14f3150ed7381
splat_clauses:
- key: source_rules
  text: '# Robe of the Archmagi Wondrous Item, Legendary (Requires Attunement by a Sorcerer, Warlock,
    or Wizard) This elegant garment is made from exquisite cloth and adorned with runes. You gain these
    benefits while wearing the robe. Armor. If you aren’t wearing armor, your base Armor Class is 15 plus
    your Dexterity modifier. Magic Resistance. You have Advantage on saving throws against spells and
    other magical effects. War Mage. Your spell save DC and spell attack bonus each increase by 2.'
  sha256: 45eb950198865354a198c1d4f46a3d78b897473ef45a075bceb10157c7706d7d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Robe of the Archmagi

<!-- SRD page 240 -->

Wondrous Item, Legendary (Requires Attunement by a\
Sorcerer, Warlock, or Wizard)\
This elegant garment is made from exquisite cloth\
and adorned with runes.\
You gain these benefits while wearing the robe.\
Armor. If you aren’t wearing armor, your base\
Armor Class is 15 plus your Dexterity modifier.\
Magic Resistance. You have Advantage on saving\
throws against spells and other magical effects.\
War Mage. Your spell save DC and spell attack bo-\
nus each increase by 2.\

Original layout: [[../../srd-5.2.1/pages/240|page 240]].
