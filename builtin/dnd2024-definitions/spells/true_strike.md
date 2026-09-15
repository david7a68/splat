---
splat_schema: 1
splat_id: ae0c00ab-82fb-57bf-aebd-52aba223b012
splat_kind: spell
splat_key: dnd2024:spells/true_strike
splat_name: True Strike
splat_origin:
  key: dnd2024:reference/spell/true_strike
splat_sources:
- title: SRD 5.2.1
  locator: pp. 171
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: divination
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a weapon with which you have proficiency and that is worth 1+ CP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/true_strike
  implementation: dnd2024:spell_true_strike
  parameters:
    attack:
      count: 1
      weapon: casting_material_weapon
      attack_ability: spellcasting
      damage_ability: spellcasting
      replace_abilities:
      - strength
      - dexterity
      damage_type_choice:
      - radiant
      - weapon_normal
      extra_damage:
        type: radiant
        dice:
          count:
            by_caster_level:
            - minimum: 1
              count: 0
            - minimum: 5
              count: 1
            - minimum: 11
              count: 2
            - minimum: 17
              count: 3
          sides: 6
          add: 0
        applies_regardless_of_chosen_damage_type: true
splat_contracts:
- id: dnd2024:spell_true_strike
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    objects:
      required: false
      type: object_list
    caster_level:
      type: integer
      required: true
    choices:
      required: false
      type: choice
    items:
      required: false
      type: item_list
  outputs:
  - effects
  - attacks
  - items
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - dice
  - attacks
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: b5109f2588d8b5dab4e728828566c750d4f724a400b60d6456f290c961b1fc29
  source_path: builtin/srd-5.2.1/spell/true_strike.md
  pages:
  - 171
  engine_gaps: &id001
  - dnd2024:spell_true_strike
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: eb4a90b4c1f9732457a4c586d91f85a0a0ceb92a069665fbc533790119cb2ba6
splat_clauses:
- key: source_rules
  text: '# True Strike Divination Cantrip (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    Self Components: S, M (a weapon with which you have proficiency and that is worth 1+ CP) Duration:
    Instantaneous Guided by a flash of magical insight, you make one attack with the weapon used in the
    spell’s casting. The attack uses your spellcasting ability for the attack and damage rolls instead
    of using Strength or Dexterity. If the attack deals damage, it can be Radiant damage or the weapon’s
    normal damage type (your choice). Cantrip Upgrade. Whether you deal Radiant damage or the weapon’s
    normal damage type, the attack deals extra Radiant damage when you reach levels 5 (1d6), 11 (2d6),
    and 17 (3d6).'
  sha256: 192f0864c2cb18a440d4fa4173ce3aeced82796a67c830a46c209338f84208e9
  status: contract_specified
splat_requires_mechanics: *id001
---

# True Strike

<!-- SRD page 171 -->

Divination Cantrip (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: S, M (a weapon with which you have profi-\
ciency and that is worth 1+ CP)\
Duration: Instantaneous\
Guided by a flash of magical insight, you make one\
attack with the weapon used in the spell’s casting.\
The attack uses your spellcasting ability for the at-\
tack and damage rolls instead of using Strength or\
Dexterity. If the attack deals damage, it can be Ra-\
diant damage or the weapon’s normal damage type\
(your choice).\
Cantrip Upgrade. Whether you deal Radiant dam-\
age or the weapon’s normal damage type, the attack\
deals extra Radiant damage when you reach levels 5\
(1d6), 11 (2d6), and 17 (3d6).\

Original layout: [[../../srd-5.2.1/pages/171|page 171]].
