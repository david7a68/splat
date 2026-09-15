---
splat_schema: 1
splat_id: 2ed70c33-8b5f-5a39-bf67-f1d52f4b6a76
splat_kind: spell
splat_key: dnd2024:spells/divine_smite
splat_name: Divine Smite
splat_origin:
  key: dnd2024:reference/spell/divine_smite
splat_sources:
- title: SRD 5.2.1
  locator: pp. 125
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: evocation
  classes:
  - paladin
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Bonus Action, which you take immediately after hitting a target with a Melee weapon or
    an Unarmed Strike
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/divine_smite
  implementation: dnd2024:spell_divine_smite
  parameters:
    target:
      source: triggering_attack_target
    trigger:
      event: attack.hit
      weapon_kinds:
      - melee_weapon
      - unarmed_strike
      timing: immediately_after
    damage:
      amount:
        dice:
          count: 2
          sides: 8
          add: 0
      type: radiant
      extra_attack_damage: true
      bonus_if_type:
      - fiend
      - undead
      type_bonus:
        dice:
          count: 1
          sides: 8
          add: 0
    scaling:
      per_slot_above: 1
      damage_dice: 1
      sides: 8
splat_contracts:
- id: dnd2024:spell_divine_smite
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    spell_attack_modifier:
      required: false
      type: integer
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - extra_attack_damage
  events:
  - spell.cast_completed
  - attack.hit
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: 78ba62c95c6a0c53bf4ca3670f3c1972a93201621d976947126ec64d7ab629ad
  source_path: builtin/srd-5.2.1/spell/divine_smite.md
  pages:
  - 125
  engine_gaps: &id001
  - dnd2024:spell_divine_smite
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 4d4661d9adb3319ce06b3aac269af5e1740c1c1c9d6a21ac3e56cdcac7d84add
splat_clauses:
- key: source_rules
  text: '# Divine Smite Level 1 Evocation (Paladin) Casting Time: Bonus Action, which you take immediately
    after hitting a target with a Melee weapon or an Unarmed Strike Range: Self Component: V Duration:
    Instantaneous The target takes an extra 2d8 Radiant damage from the attack. The damage increases by
    1d8 if the target is a Fiend or an Undead. Using a Higher-Level Spell Slot. The damage increases by
    1d8 for each spell slot level above 1.'
  sha256: 7aed555ca24fa57f202932b7096ca0b7d56a4bdf33b045cb45cb63b376744df0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Divine Smite

<!-- SRD page 125 -->

Level 1 Evocation (Paladin)\
Casting Time: Bonus Action, which you take immedi-\
ately after hitting a target with a Melee weapon or an\
Unarmed Strike\
Range: Self\
Component: V\
Duration: Instantaneous\
The target takes an extra 2d8 Radiant damage from\
the attack. The damage increases by 1d8 if the tar-\
get is a Fiend or an Undead.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/125|page 125]].
