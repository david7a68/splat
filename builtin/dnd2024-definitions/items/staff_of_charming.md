---
splat_schema: 1
splat_id: 948ae061-ee5e-5a75-b00a-ecc98ed4cbfc
splat_kind: item
splat_key: dnd2024:items/staff_of_charming
splat_name: Staff of Charming
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_charming
splat_sources:
- title: SRD 5.2.1
  locator: pp. 245
  url: https://www.dndbeyond.com/srd
splat_item:
  category: staff
  rarity: rare
  equipment:
    attunement: required
    attunement_requirement_text: a Bard, Cleric, Druid, Sorcerer, Warlock, or Wizard
splat_rule:
  mechanic: dnd2024:items/staff_of_charming
  implementation: dnd2024:item_staff_of_charming
  parameters:
    equipment_state: held
    charges:
      maximum: 10
      initial: 10
      recovery:
        event: dawn
        amount:
          dice: 1
          sides: 8
          bonus: 2
    spells:
    - spell:
        key: dnd2024:spells/charm_person
      cost: 1
      save_dc: user_spell_save_dc
    - spell:
        key: dnd2024:spells/command
      cost: 1
      save_dc: user_spell_save_dc
    - spell:
        key: dnd2024:spells/comprehend_languages
      cost: 1
      save_dc: user_spell_save_dc
    reflect:
      action: reaction
      trigger: successful_save_against_enchantment_spell_targeting_only_user
      cost: 1
      target: original_caster
      cast_as: user
    resist:
      trigger: failed_save_against_enchantment_spell_targeting_only_user
      resource:
        uses: 1
        recovery: dawn
      replace_result: success
      action: none
    last_charge:
      roll:
        dice: 1
        sides: 20
        bonus: 0
      on_value: 1
      effect: destroy_item
    attunement:
      required: true
      eligibility:
        class_any:
        - bard
        - cleric
        - druid
        - sorcerer
        - warlock
        - wizard
splat_contracts:
- id: dnd2024:item_staff_of_charming
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
    targets:
      required: false
      type: participant_list
    event:
      required: false
      type: event
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
  source_sha256: 788d10e9d80bb5464dbc67892f228050592eed7c34c2dc98717c0ec32e203aaf
  source_path: builtin/srd-5.2.1/magic_item/staff_of_charming.md
  pages:
  - 245
  engine_gaps: &id001
  - dnd2024:item_staff_of_charming
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: bbeed81343cb838c16d97845fad6d9261b9f35413e3413e80795ea18e6fbbb12
splat_clauses:
- key: source_rules
  text: '# Staff of Charming Staff, Rare (Requires Attunement by a Bard, Cleric, Druid, Sorcerer, Warlock,
    or Wizard) This staff has 10 charges. While holding the staff, you can use any of its properties:
    Cast Spell. You can expend 1 of the staff’s charges to cast Charm Person, Command, or Comprehend Languages
    from it using your spell save DC. Reflect Enchantment. If you succeed on a saving throw against an
    Enchantment spell that targets only you, you can take a Reaction to expend 1 charge from the staff
    and turn the spell back on its caster as if you had cast the spell. Resist Enchantment. If you fail
    a saving throw against an Enchantment spell that targets only you, you can turn your failed save into
    a successful one. You can’t use this property of the staff again until the next dawn. Regaining Charges.
    The staff regains 1d8 + 2 expended charges daily at dawn. If you expend the last charge, roll 1d20.
    On a 1, the staff crumbles to dust and is destroyed.'
  sha256: c5a0347e58244ea63974b6975e8c80ce83b5b44d723410a36a080dee82d1188f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staff of Charming

<!-- SRD page 245 -->

Staff, Rare (Requires Attunement by a Bard, Cleric,\
Druid, Sorcerer, Warlock, or Wizard)\
This staff has 10 charges. While holding the staff,\
you can use any of its properties:\
Cast Spell. You can expend 1 of the staff’s charges\
to cast Charm Person, Command, or Comprehend\
Languages from it using your spell save DC.\
Reflect Enchantment. If you succeed on a saving\
throw against an Enchantment spell that targets\
only you, you can take a Reaction to expend 1\
charge from the staff and turn the spell back on\
its caster as if you had cast the spell.\
Resist Enchantment. If you fail a saving throw\
against an Enchantment spell that targets only\
you, you can turn your failed save into a success-\
ful one. You can’t use this property of the staff\
again until the next dawn.\
Regaining Charges. The staff regains 1d8 + 2 ex-\
pended charges daily at dawn. If you expend the last\
charge, roll 1d20. On a 1, the staff crumbles to dust\
and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/245|page 245]].
