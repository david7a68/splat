---
splat_schema: 1
splat_id: 244810ae-b90d-5e81-82bb-418a00484de1
splat_kind: item
splat_key: dnd2024:items/plate_armor_of_etherealness
splat_name: Plate Armor of Etherealness
splat_origin:
  key: dnd2024:reference/magic_item/plate_armor_of_etherealness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/plate_armor_of_etherealness
  implementation: dnd2024:item_plate_armor_of_etherealness
  parameters:
    equipment_state: worn
    activation:
      action: magic
      command_word: true
      resource:
        uses: 1
        recovery: dawn
      gain_spell_effect:
        spell:
          key: dnd2024:spells/etherealness
      end_on:
      - armor_removed
      - magic_action_repeat_command_word
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_plate_armor_of_etherealness
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    locations:
      required: false
      type: location_list
  outputs:
  - effects
  - item_state
  events:
  - item.used
  - spell.cast_completed
  capabilities:
  - equipment
  - item_resources
  - effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 0af676748f01367005fb2fd3cdc81e4cf7cf721461b3392b19adb51f63a710ca
  source_path: builtin/srd-5.2.1/magic_item/plate_armor_of_etherealness.md
  pages:
  - 234
  engine_gaps: &id001
  - dnd2024:item_plate_armor_of_etherealness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 93da8f7ac6e91dce747dc0b8d519b7928884a0d193154c3a4bc0ba2ed22de43e
splat_clauses:
- key: source_rules
  text: '# Plate Armor of Etherealness Armor (Half Plate Armor or Plate Armor), Legendary (Requires Attunement)
    While you’re wearing this armor, you can take a Magic action and use a command word to gain the effect
    of the Etherealness spell. The spell ends immediately if you remove the armor or take a Magic action
    to repeat the command word. This property of the armor can’t be used again until the next dawn.'
  sha256: a0742e53f27fdd0807ded036421a61bbcfbce164a03b2ede3778f43fbf5c85a2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Plate Armor of Etherealness

<!-- SRD page 234 -->

Armor (Half Plate Armor or Plate Armor), Legendary\
(Requires Attunement)\
While you’re wearing this armor, you can take a\
Magic action and use a command word to gain the\
effect of the Etherealness spell. The spell ends imme-\
diately if you remove the armor or take a Magic ac-\
tion to repeat the command word. This property of\
the armor can’t be used again until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]].
