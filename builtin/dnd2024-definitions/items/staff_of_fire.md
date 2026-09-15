---
splat_schema: 1
splat_id: 5e744cdd-c403-5296-9197-622968899bba
splat_kind: item
splat_key: dnd2024:items/staff_of_fire
splat_name: Staff of Fire
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_fire
splat_sources:
- title: SRD 5.2.1
  locator: pp. 245
  url: https://www.dndbeyond.com/srd
splat_item:
  category: staff
  rarity: very_rare
  equipment:
    attunement: required
    attunement_requirement_text: a Druid, Sorcerer, Warlock, or Wizard
splat_rule:
  mechanic: dnd2024:items/staff_of_fire
  implementation: dnd2024:item_staff_of_fire
  parameters:
    equipment_state: held
    damage_resistance: fire
    charges:
      maximum: 10
      initial: 10
      recovery:
        event: dawn
        amount:
          expression: 1d6+4
          scope: use
        cap: maximum
    spell_save_dc: user_spell_save_dc
    spell_grants:
    - spell:
        key: dnd2024:spells/burning_hands
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/wall_of_fire
      charge_cost: 4
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/fireball
      charge_cost: 3
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    last_charge:
      roll:
        expression: 1d20
        scope: use
      'on': 1
      effect: destroyed
    attunement:
      required: true
      eligible_classes:
      - druid
      - sorcerer
      - warlock
      - wizard
    common_rules:
    - key: dnd2024:rules/magic_item_categories_staffs
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_staff_of_fire
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
  - spell.cast_pending
  - time.dawn
  - equipment.changed
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: b9c2aef7dc5f7ec6551ea5044de03cc9f77c47ced010c0280808da25e523a5fc
  source_path: builtin/srd-5.2.1/magic_item/staff_of_fire.md
  pages:
  - 245
  engine_gaps: &id001
  - dnd2024:item_staff_of_fire
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 4e81f7669bd34710163ce84ec5940f727f4f36d1cdb279b7a78c4322be12ec36
splat_clauses:
- key: source_rules
  text: '# Staff of Fire Staff, Very Rare (Requires Attunement by a Druid, Sorcerer, Warlock, or Wizard)
    You have Resistance to Fire damage while you hold this staff. Spells. The staff has 10 charges. While
    holding the staff, you can cast one of the spells on the following table from it, using your spell
    save DC. The table indicates how many charges you must expend to cast the spell. Charge Charge Spell
    Cost Spell Cost Burning Hands 1 Wall of Fire 4 Fireball 3 Regaining Charges. The staff regains 1d6
    + 4 expended charges daily at dawn. If you expend the last charge, roll 1d20. On a 1, the staff crumbles
    into cinders and is destroyed.'
  sha256: e3f5fc3877c9a0cecc483a9d46910b464c6b971d19a14912fe6aed523f4a4aaa
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staff of Fire

<!-- SRD page 245 -->

Staff, Very Rare (Requires Attunement by a Druid,\
Sorcerer, Warlock, or Wizard)\
You have Resistance to Fire damage while you hold\
this staff.\
Spells. The staff has 10 charges. While holding the\
staff, you can cast one of the spells on the following\
table from it, using your spell save DC. The table in-\
dicates how many charges you must expend to cast\
the spell.\
Charge Charge\
Spell Cost Spell Cost\
Burning Hands 1 Wall of Fire 4\
Fireball 3\
Regaining Charges. The staff regains 1d6 + 4\
expended charges daily at dawn. If you expend the\
last charge, roll 1d20. On a 1, the staff crumbles into\
cinders and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/245|page 245]].
