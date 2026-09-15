---
splat_schema: 1
splat_id: b6a26867-9df9-5f89-86a4-24f67cb6926e
splat_kind: item
splat_key: dnd2024:items/staff_of_swarming_insects
splat_name: Staff of Swarming Insects
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_swarming_insects
splat_sources:
- title: SRD 5.2.1
  locator: pp. 246
  url: https://www.dndbeyond.com/srd
splat_item:
  category: staff
  rarity: rare
  equipment:
    attunement: required
    attunement_requirement_text: a Bard, Cleric, Druid, Sorcerer, Warlock, or Wizard
splat_rule:
  mechanic: dnd2024:items/staff_of_swarming_insects
  implementation: dnd2024:item_staff_of_swarming_insects
  parameters:
    equipment_state: held
    charges:
      maximum: 10
      initial: 10
      recovery:
        event: dawn
        amount:
          dice: 1
          sides: 6
          bonus: 4
    insect_cloud:
      action: magic
      cost: 1
      shape: emanation
      radius_ft: 30
      origin: user
      duration:
        value: 10
        unit: minute
      insects_harmless: true
      area: heavily_obscured
      vision_exception: user
      end_on: strong_wind
    spells:
    - spell:
        key: dnd2024:spells/giant_insect
      cost: 4
      save_dc: user_spell_save_dc
      attack_modifier: user_spell_attack_modifier
    - spell:
        key: dnd2024:spells/insect_plague
      cost: 5
      save_dc: user_spell_save_dc
      attack_modifier: user_spell_attack_modifier
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
- id: dnd2024:item_staff_of_swarming_insects
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
  source_sha256: 290b3cad435dd1d58e1a204c5cc379377797e74691ec77d13735b6dbb48fff99
  source_path: builtin/srd-5.2.1/magic_item/staff_of_swarming_insects.md
  pages:
  - 246
  engine_gaps: &id001
  - dnd2024:item_staff_of_swarming_insects
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: f58615adadf3c464e03d7f15d86c5c34332f29425db6414a07243a861976c54f
splat_clauses:
- key: source_rules
  text: '# Staff of Swarming Insects Staff, Rare (Requires Attunement by a Bard, Cleric, Druid, Sorcerer,
    Warlock, or Wizard) This staff has 10 charges. Insect Cloud. While holding the staff, you can take
    a Magic action and expend 1 charge to cause a swarm of harmless flying insects to fill a 30- foot
    Emanation originating from you. The insects remain for 10 minutes, making the area Heavily Obscured
    for creatures other than you. A strong wind (like that created by Gust of Wind) disperses the swarm
    and ends the effect. Spells. While holding the staff, you can cast one of the spells on the following
    table from it, using your spell save DC and spell attack modifier. The table indicates how many charges
    you must expend to cast the spell. Spell Charge Cost Giant Insect 4 Insect Plague 5 Regaining Charges.
    The staff regains 1d6 + 4 expended charges daily at dawn. If you expend the last charge, roll 1d20.
    On a 1, a swarm of insects consumes and destroys the staff, then disperses.'
  sha256: babb53a2d9583975bbebe47547ed9ab5847c87faeab5411b7c62ae56d0d54bec
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staff of Swarming Insects

<!-- SRD page 246 -->

Staff, Rare (Requires Attunement by a Bard, Cleric,\
Druid, Sorcerer, Warlock, or Wizard)\
This staff has 10 charges.\
Insect Cloud. While holding the staff, you can\
take a Magic action and expend 1 charge to cause\
a swarm of harmless flying insects to fill a 30-\
foot Emanation originating from you. The insects\
remain for 10 minutes, making the area Heavily\
Obscured for creatures other than you. A strong\
wind (like that created by Gust of Wind) disperses\
the swarm and ends the effect.\
Spells. While holding the staff, you can cast one of\
the spells on the following table from it, using your\
spell save DC and spell attack modifier. The table in-\
dicates how many charges you must expend to cast\
the spell.\
Spell Charge\
Cost\
Giant Insect 4\
Insect Plague 5\
Regaining Charges. The staff regains 1d6 + 4\
expended charges daily at dawn. If you expend the\
last charge, roll 1d20. On a 1, a swarm of insects\
consumes and destroys the staff, then disperses.\

Original layout: [[../../srd-5.2.1/pages/246|page 246]].
