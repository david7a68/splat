---
splat_schema: 1
splat_id: 43b29943-2868-5c23-8e20-df98ac3fb8dc
splat_kind: item
splat_key: dnd2024:items/quarterstaff_of_the_acrobat
splat_name: Quarterstaff of the Acrobat
splat_origin:
  key: dnd2024:reference/magic_item/quarterstaff_of_the_acrobat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 237
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/quarterstaff_of_the_acrobat
  implementation: dnd2024:item_quarterstaff_of_the_acrobat
  parameters:
    weapon_bonus:
      attack: 2
      damage: 2
    equipment_state: held
    light:
      kind: dim
      color: green
      radius_ft: 10
      enable:
      - bonus_action
      - after_initiative_roll
      disable: bonus_action
    change_form:
      action: bonus_action
      options:
        rod:
          length_inches: 6
        pole:
          length_ft: 10
        quarterstaff: {}
      elongation_limited_by: surrounding_space
    form_properties:
      quarterstaff:
        acrobatics: advantage
        thrown:
          normal_range_ft: 30
          long_range_ft: 120
          return_to_hand: immediately_after_ranged_attack
        deflection:
          action: reaction
          trigger: hit_by_attack
          armor_class_bonus: 5
          scope: triggering_attack
          can_turn_hit_to_miss: true
          uses: 1
          recovery:
          - short_rest
          - long_rest
      pole:
        acrobatics: advantage
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_quarterstaff_of_the_acrobat
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
    event:
      required: false
      type: event
  outputs:
  - modifiers
  - effects
  - damage
  - item_state
  events:
  - equipment.state_changed
  - attack.resolved
  - saving_throw.resolved
  capabilities:
  - equipment
  - combat_reactions
  - effects
  - damage
  - item_resources
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 72adcef562548173cce6660aba5b1ceab40e97f6efc5b10085fb1b5b0b1daf71
  source_path: builtin/srd-5.2.1/magic_item/quarterstaff_of_the_acrobat.md
  pages:
  - 237
  engine_gaps: &id001
  - dnd2024:item_quarterstaff_of_the_acrobat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 6bc3717827aeddb50baf8b3ea081412b274d145ab08745ae8b4c3e81d28131c5
splat_clauses:
- key: source_rules
  text: '# Quarterstaff of the Acrobat Weapon (Quarterstaff), Very Rare (Requires Attunement) You have
    a +2 bonus to attack rolls and damage rolls made with this magic weapon. While holding this weapon,
    you can cause it to emit green Dim Light out to 10 feet, either as a Bonus Action or after you roll
    Initiative, or you can extinguish the light as a Bonus Action. While holding this weapon, you can
    take a Bonus Action to alter its form, turning it into a 6-inch rod (for ease of storage) or a 10-foot
    pole, or reverting it a Quarterstaff; the weapon will elongate only as far as the surrounding space
    allows. In certain forms, the weapon has the following additional properties. Acrobatic Assist (Quarterstaff
    and 10-Foot Pole Forms Only). While holding this weapon, you have Advantage on Dexterity (Acrobatics)
    checks. Attack Deflection (Quarterstaff Form Only). When you are hit by an attack while holding the
    weapon, you can take a Reaction to twirl the weapon around you, gaining a +5 bonus to your Armor Class
    against the triggering attack, potentially causing the attack to miss you. You can’t use this property
    again until you finish a Short or Long Rest. Ranged Weapon (Quarterstaff Form Only). This weapon has
    the Thrown property with a normal range of 30 feet and a long range of 120 feet. Immediately after
    you make a ranged attack with the weapon, it flies back to your hand.'
  sha256: a6b32f3299cb50d05a8306fb3d62b84cecebeabfbc6020d8ce8bf766356deff3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Quarterstaff of the Acrobat

<!-- SRD page 237 -->

Weapon (Quarterstaff), Very Rare (Requires\
Attunement)\
You have a +2 bonus to attack rolls and damage rolls\
made with this magic weapon.\
While holding this weapon, you can cause it to\
emit green Dim Light out to 10 feet, either as a Bo-\
nus Action or after you roll Initiative, or you can\
extinguish the light as a Bonus Action.\
While holding this weapon, you can take a Bonus\
Action to alter its form, turning it into a 6-inch rod\
(for ease of storage) or a 10-foot pole, or reverting it\
a Quarterstaff; the weapon will elongate only as far\
as the surrounding space allows.\
In certain forms, the weapon has the following ad-\
ditional properties.\
Acrobatic Assist (Quarterstaff and 10-Foot Pole\
Forms Only). While holding this weapon, you have\
Advantage on Dexterity (Acrobatics) checks.\
Attack Deflection (Quarterstaff Form Only).\
When you are hit by an attack while holding the\
weapon, you can take a Reaction to twirl the\
weapon around you, gaining a +5 bonus to your Ar-\
mor Class against the triggering attack, potentially\
causing the attack to miss you. You can’t use this\
property again until you finish a Short or Long Rest.\
Ranged Weapon (Quarterstaff Form Only). This\
weapon has the Thrown property with a normal\
range of 30 feet and a long range of 120 feet. Im-\
mediately after you make a ranged attack with the\
weapon, it flies back to your hand.\

Original layout: [[../../srd-5.2.1/pages/237|page 237]].
