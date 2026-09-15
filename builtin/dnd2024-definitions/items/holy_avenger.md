---
splat_schema: 1
splat_id: 9f2a75bc-7610-58ac-9e92-56b38e9f6025
splat_kind: item
splat_key: dnd2024:items/holy_avenger
splat_name: Holy Avenger
splat_origin:
  key: dnd2024:reference/magic_item/holy_avenger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: legendary
  equipment:
    attunement: required
    attunement_requirement_text: a Paladin
splat_rule:
  mechanic: dnd2024:items/holy_avenger
  implementation: dnd2024:item_holy_avenger
  parameters:
    attack_roll_bonus: 3
    damage_roll_bonus: 3
    scope: attacks_with_source_item
    on_hit:
      target_types:
      - fiend
      - undead
      damage:
        roll:
          expression: 2d10
          scope: attack
        type: radiant
    aura:
      equipment_state: held_drawn
      shape: emanation
      origin: user
      radius_ft: 10
      radius_override:
        paladin_level_minimum: 17
        radius_ft: 30
      targets:
      - self
      - friendly_creatures
      save_advantage_against:
      - spells
      - other_magical_effects
    attunement:
      required: true
      eligible_classes:
      - paladin
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_holy_avenger
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
  - attack.roll_pending
  - attack.hit
  - equipment.changed
  - position.changed
  - saving_throw.pending
  - class.level_changed
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - roll_resolution
  - spatial_state
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 11271e654cb72e466f071f3b8cf48711c7bba384f09173b3cc284643166cc410
  source_path: builtin/srd-5.2.1/magic_item/holy_avenger.md
  pages:
  - 226
  engine_gaps: &id001
  - dnd2024:item_holy_avenger
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: e661f4ef70a235e758998eecf9b1c589285627c7ef4e225c02ba7c4e2c0f01d9
splat_clauses:
- key: source_rules
  text: '# Holy Avenger Weapon (Any Simple or Martial), Legendary (Requires Attunement by a Paladin) You
    gain a +3 bonus to attack rolls and damage rolls made with this magic weapon. When you hit a Fiend
    or an Undead with it, that creature takes an extra 2d10 Radiant damage. While you hold the drawn weapon,
    it creates a 10-foot Emanation originating from you. You and all creatures Friendly to you in the
    Emanation have Advantage on saving throws against spells and other magical effects. If you have 17
    or more levels in the Paladin class, the size of the Emanation increases to 30 feet.'
  sha256: d698a0e291c2113690ca3a9ddb5eafa3527175b45137c64ce29f76b4f03bbaa8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Holy Avenger

<!-- SRD page 226 -->

Weapon (Any Simple or Martial), Legendary (Requires\
Attunement by a Paladin)\
You gain a +3 bonus to attack rolls and damage rolls\
made with this magic weapon. When you hit a Fiend\
or an Undead with it, that creature takes an extra\
2d10 Radiant damage.\
While you hold the drawn weapon, it creates a\
10-foot Emanation originating from you. You and all\
creatures Friendly to you in the Emanation have Ad-\
vantage on saving throws against spells and other\
magical effects. If you have 17 or more levels in the\
Paladin class, the size of the Emanation increases to\
30 feet.\

Original layout: [[../../srd-5.2.1/pages/226|page 226]].
