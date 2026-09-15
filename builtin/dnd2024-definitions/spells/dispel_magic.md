---
splat_schema: 1
splat_id: 0091936d-2449-5293-bc84-d13f07ab9c8b
splat_kind: spell
splat_key: dnd2024:spells/dispel_magic
splat_name: Dispel Magic
splat_origin:
  key: dnd2024:reference/spell/dispel_magic
splat_sources:
- title: SRD 5.2.1
  locator: pp. 124
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: abjuration
  classes:
  - bard
  - cleric
  - druid
  - paladin
  - ranger
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
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/dispel_magic
  implementation: dnd2024:spell_dispel_magic
  parameters:
    target:
      kind: creature_object_or_magical_effect
      count: 1
      visible_to_caster: false
    ongoing_spells:
      each: true
      automatic_end_max_level:
        binding: cast_level
      higher_level:
        check_ability: spellcasting
        dc:
          op: add
          args:
          - 10
          - binding: affected_spell_level
        success: end_spell
splat_contracts:
- id: dnd2024:spell_dispel_magic
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
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
    affected_spell_level:
      type: integer
      required: true
  outputs:
  - checks
  - ended_spells
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - inventory
  - checks
splat_conversion:
  status: specified
  category: spell
  source_sha256: a7613ef509554f37c461a40cea762282fa9e0cde0d9f8b501986568e4844dafb
  source_path: builtin/srd-5.2.1/spell/dispel_magic.md
  pages:
  - 124
  engine_gaps: &id001
  - dnd2024:spell_dispel_magic
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: b77486b1fdb48f147543f6ad9bb971b8c549c0e36eac1a5e752828f68b3b79bf
splat_clauses:
- key: source_rules
  text: '# Dispel Magic Level 3 Abjuration (Bard, Cleric, Druid, Paladin, Ranger, Sorcerer, Warlock, Wizard)
    Casting Time: Action Range: 120 feet Components: V, S Duration: Instantaneous Choose one creature,
    object, or magical effect within range. Any ongoing spell of level 3 or lower on the target ends.
    For each ongoing spell of level 4 or higher on the target, make an ability check using your spellcasting
    ability (DC 10 plus that spell’s level). On a successful check, the spell ends. Using a Higher-Level
    Spell Slot. You automatically end a spell on the target if the spell’s level is equal to or less than
    the level of the spell slot you use.'
  sha256: affde403e3e379c2bd8d63bc98841ff2c1dd7909fafdc22a555f97ab2d9f1989
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dispel Magic

<!-- SRD page 124 -->

Level 3 Abjuration (Bard, Cleric, Druid, Paladin,\
Ranger, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous\
Choose one creature, object, or magical effect\
within range. Any ongoing spell of level 3 or lower\
on the target ends. For each ongoing spell of level 4\
or higher on the target, make an ability check using\
your spellcasting ability (DC 10 plus that spell’s\
level). On a successful check, the spell ends.\
Using a Higher-Level Spell Slot. You automati-\
cally end a spell on the target if the spell’s level is\
equal to or less than the level of the spell slot you\
use.\

Original layout: [[../../srd-5.2.1/pages/124|page 124]].
