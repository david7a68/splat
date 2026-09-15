---
splat_schema: 1
splat_id: 2f5179e0-f2fa-5e40-ab0b-947f031eb8e2
splat_kind: spell
splat_key: dnd2024:spells/magic_weapon
splat_name: Magic Weapon
splat_origin:
  key: dnd2024:reference/spell/magic_weapon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 146, 147
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - paladin
  - ranger
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 hour
  casting_time: Bonus Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/magic_weapon
  implementation: dnd2024:spell_magic_weapon
  parameters:
    target:
      kind: weapon
      count: 1
      visible_to_caster: false
      touch: true
      magical: false
    magic_weapon: true
    attack_and_damage_bonus_by_slot:
      '2': 1
      '3': 2
      '4': 2
      '5': 2
      '6': 3
      '7': 3
      '8': 3
      '9': 3
    end:
      event: caster.recasts_same_spell
splat_contracts:
- id: dnd2024:spell_magic_weapon
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
  - weapon_enchantment
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: e85f680b3d9f8a9805d74bd2e44cff44833aea5759c6dd079882cd5081aa6e21
  source_path: builtin/srd-5.2.1/spell/magic_weapon.md
  pages:
  - 146
  - 147
  engine_gaps: &id001
  - dnd2024:spell_magic_weapon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: a247cb5396e6bae699476a2664d05024d6586959daa98f1a2f0c42503ee06f45
splat_clauses:
- key: source_rules
  text: '# Magic Weapon Level 2 Transmutation (Paladin, Ranger, Sorcerer, Wizard) Casting Time: Bonus
    Action Range: Touch Components: V, S Duration: 1 hour You touch a nonmagical weapon. Until the spell
    ends, that weapon becomes a magic weapon with a +1 bonus to attack rolls and damage rolls. The spell
    ends early if you cast it again. Using a Higher-Level Spell Slot. The bonus increases to +2 with a
    level 3–5 spell slot. The bonus increases to +3 with a level 6+ spell slot.'
  sha256: 42ad1819fa35f8e0cc23125f0eee0357353571be724c00d33d86b1ba5aaf345a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magic Weapon

<!-- SRD page 146 -->

Level 2 Transmutation (Paladin, Ranger, Sorcerer,\
Wizard)\
Casting Time: Bonus Action\
Range: Touch\

<!-- SRD page 147 -->

Components: V, S\
Duration: 1 hour\
You touch a nonmagical weapon. Until the spell\
ends, that weapon becomes a magic weapon with a\
+1 bonus to attack rolls and damage rolls. The spell\
ends early if you cast it again.\
Using a Higher-Level Spell Slot. The bonus in-\
creases to +2 with a level 3–5 spell slot. The bonus\
increases to +3 with a level 6+ spell slot.\

Original layout: [[../../srd-5.2.1/pages/146|page 146]], [[../../srd-5.2.1/pages/147|page 147]].
