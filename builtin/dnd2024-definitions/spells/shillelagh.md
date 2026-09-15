---
splat_schema: 1
splat_id: b80a91c5-1e35-50d3-bbba-51b10ce494d2
splat_kind: spell
splat_key: dnd2024:spells/shillelagh
splat_name: Shillelagh
splat_origin:
  key: dnd2024:reference/spell/shillelagh
splat_sources:
- title: SRD 5.2.1
  locator: pp. 162
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: transmutation
  classes:
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 minute
  casting_time: Bonus Action
  ritual: false
  range:
    mode: self
  material:
    description: mistletoe
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/shillelagh
  implementation: dnd2024:spell_shillelagh
  parameters:
    target:
      kind: held_weapon
      allowed:
      - club
      - quarterstaff
    melee_attacks:
      optional_ability_replacement: spellcasting_instead_of_strength
      applies_to:
      - attack_roll
      - damage_roll
      damage_dice_by_character_level:
        '1':
          dice:
            count: 1
            sides: 8
            add: 0
        '5':
          dice:
            count: 1
            sides: 10
            add: 0
        '11':
          dice:
            count: 1
            sides: 12
            add: 0
        '17':
          dice:
            count: 2
            sides: 6
            add: 0
      damage_type_choice:
      - force
      - weapon_normal
    end:
      events:
      - caster.recasts_same_spell
      - caster.lets_go_of_weapon
splat_contracts:
- id: dnd2024:spell_shillelagh
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
    character_level:
      type: integer
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
  - weapon_effect
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
  - inventory
  - choices
splat_conversion:
  status: specified
  category: spell
  source_sha256: e45dcf4f260ef62f40a0609eae6b37327de9107018e9b8feb9cdb27fc3a5e56d
  source_path: builtin/srd-5.2.1/spell/shillelagh.md
  pages:
  - 162
  engine_gaps: &id001
  - dnd2024:spell_shillelagh
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: c35b5e45ac382f955e724233611b697bd501e3a3b2dbf92d498ff98fe0a35b0a
splat_clauses:
- key: source_rules
  text: '# Shillelagh Transmutation Cantrip (Druid) Casting Time: Bonus Action Range: Self Components:
    V, S, M (mistletoe) Duration: 1 minute A Club or Quarterstaff you are holding is imbued with nature’s
    power. For the duration, you can use your spellcasting ability instead of Strength for the attack
    and damage rolls of melee attacks using that weapon, and the weapon’s damage die becomes a d8. If
    the attack deals damage, it can be Force damage or the weapon’s normal damage type (your choice).
    The spell ends early if you cast it again or if you let go of the weapon. Cantrip Upgrade. The damage
    die changes when you reach levels 5 (d10), 11 (d12), and 17 (2d6).'
  sha256: 07567f48cca33732c2d4e8f0b92fc40cddcd302d70b35879f04ca655d23a8198
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shillelagh

<!-- SRD page 162 -->

Transmutation Cantrip (Druid)\
Casting Time: Bonus Action\
Range: Self\
Components: V, S, M (mistletoe)\
Duration: 1 minute\
A Club or Quarterstaff you are holding is imbued\
with nature’s power. For the duration, you can use\
your spellcasting ability instead of Strength for the\
attack and damage rolls of melee attacks using that\
weapon, and the weapon’s damage die becomes\
a d8. If the attack deals damage, it can be Force\
damage or the weapon’s normal damage type (your\
choice).\
The spell ends early if you cast it again or if you let\
go of the weapon.\
Cantrip Upgrade. The damage die changes when\
you reach levels 5 (d10), 11 (d12), and 17 (2d6).\

Original layout: [[../../srd-5.2.1/pages/162|page 162]].
