---
splat_schema: 1
splat_id: 0d7805ca-c716-5d95-a3c0-471974427771
splat_kind: spell
splat_key: dnd2024:spells/poison_spray
splat_name: Poison Spray
splat_origin:
  key: dnd2024:reference/spell/poison_spray
splat_sources:
- title: SRD 5.2.1
  locator: pp. 153
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: necromancy
  classes:
  - druid
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/poison_spray
  implementation: dnd2024:spell_poison_spray
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: false
    attack:
      kind: ranged_spell
      hit:
        damage:
          amount:
            dice:
              count: 1
              sides: 12
              add: 0
          type: poison
    cantrip_dice_by_character_level:
      '1': 1
      '5': 2
      '11': 3
      '17': 4
splat_contracts:
- id: dnd2024:spell_poison_spray
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    spell_attack_modifier:
      type: integer
      required: true
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
  - attack_result
  - damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
splat_conversion:
  status: specified
  category: spell
  source_sha256: 0c37c5bd07298d1c87b428b8654dc9ab554f2ea8d983e58fdd2bb701f282eb68
  source_path: builtin/srd-5.2.1/spell/poison_spray.md
  pages:
  - 153
  engine_gaps: &id001
  - dnd2024:spell_poison_spray
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: ed62f70fcca8caa19ebf8daf947dee64cf395a6eb65d10c1b87cee466fe18b59
splat_clauses:
- key: source_rules
  text: '# Poison Spray Necromancy Cantrip (Druid, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    30 feet Components: V, S Duration: Instantaneous You spray toxic mist at a creature within range.
    Make a ranged spell attack against the target. On a hit, the target takes 1d12 Poison damage. Cantrip
    Upgrade. The damage increases by 1d12 when you reach levels 5 (2d12), 11 (3d12), and 17 (4d12).'
  sha256: 1ca75fc553c74424a8f18c8d215550feb7b41d6fe4ae774795bab53d6fe042c8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Poison Spray

<!-- SRD page 153 -->

Necromancy Cantrip (Druid, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Instantaneous\
You spray toxic mist at a creature within range.\
Make a ranged spell attack against the target. On a\
hit, the target takes 1d12 Poison damage.\
Cantrip Upgrade. The damage increases by 1d12\
when you reach levels 5 (2d12), 11 (3d12), and 17\
(4d12).\

Original layout: [[../../srd-5.2.1/pages/153|page 153]].
