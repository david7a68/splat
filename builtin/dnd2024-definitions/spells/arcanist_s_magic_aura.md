---
splat_schema: 1
splat_id: 82c626da-f5c6-5dc8-939d-560e4030f7e3
splat_kind: spell
splat_key: dnd2024:spells/arcanist_s_magic_aura
splat_name: Arcanist’s Magic Aura
splat_origin:
  key: dnd2024:reference/spell/arcanist_s_magic_aura
splat_sources:
- title: SRD 5.2.1
  locator: pp. 110, 111
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: illusion
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 24 hours
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a small square of silk
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/arcanist_s_magic_aura
  implementation: dnd2024:spell_arcanist_s_magic_aura
  parameters:
    targeting:
      options:
      - kind: creature
        willing: true
        mode: mask
      - kind: object
        worn: false
        carried: false
        mode: false_aura
      touch: true
    mask:
      choice: creature_type
      excluded: target_actual_type
      affected_systems:
      - spells
      - magical_effects
      treated_type: chosen_type
    false_aura:
      affected_systems:
      - magic_aura_detection_spells
      - magic_aura_detection_effects
      choices:
      - nonmagical_appears_magical
      - magical_appears_nonmagical
      - chosen_school_aura
    persistence:
      same_target: true
      frequency: daily
      consecutive_days: 30
      resulting_duration: until_dispelled
    spell_lifetime:
      concentration: false
      maximum:
        value: 24
        unit: hour
splat_contracts:
- id: dnd2024:spell_arcanist_s_magic_aura
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
      type: object_list
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  - time.elapsed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 02b68d9fd119be15ec29244c753d4a04196b84afb69786705f02e71a91bdeeb1
  source_path: builtin/srd-5.2.1/spell/arcanist_s_magic_aura.md
  pages:
  - 110
  - 111
  engine_gaps: &id001
  - dnd2024:spell_arcanist_s_magic_aura
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 417dcdb14259ff04bfa5106b2bf90b719ec917926fac0e7363191dea012a2e74
splat_clauses:
- key: source_rules
  text: '# Arcanist’s Magic Aura Level 2 Illusion (Wizard) Casting Time: Action Range: Touch Components:
    V, S, M (a small square of silk) Duration: 24 hours With a touch, you place an illusion on a willing
    creature or an object that isn’t being worn or carried. A creature gains the Mask effect below, and
    an object gains the False Aura effect below. The effect lasts for the duration. If you cast the spell
    on the same target every day for 30 days, the illusion lasts until dispelled. Mask (Creature). Choose
    a creature type other than the target’s actual type. Spells and other magical effects treat the target
    as if it were a creature of the chosen type. False Aura (Object). You change the way the target appears
    to spells and magical effects that detect magical auras, such as Detect Magic. You can make a nonmagical
    object appear magical, make a magic item appear nonmagical, or change the object’s aura so that it
    appears to belong to a school of magic you choose.'
  sha256: 2c71b910f210adab82951584e3a43b50fd2005cd8080afdb2e99024a3e25a2f5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Arcanist’s Magic Aura

<!-- SRD page 110 -->

Level 2 Illusion (Wizard)\
Casting Time: Action\
Range: Touch\

<!-- SRD page 111 -->

Components: V, S, M (a small square of silk)\
Duration: 24 hours\
With a touch, you place an illusion on a willing crea-\
ture or an object that isn’t being worn or carried. A\
creature gains the Mask effect below, and an object\
gains the False Aura effect below. The effect lasts\
for the duration. If you cast the spell on the same\
target every day for 30 days, the illusion lasts until\
dispelled.\
Mask (Creature). Choose a creature type other\
than the target’s actual type. Spells and other magi-\
cal effects treat the target as if it were a creature of\
the chosen type.\
False Aura (Object). You change the way the tar-\
get appears to spells and magical effects that detect\
magical auras, such as Detect Magic. You can make\
a nonmagical object appear magical, make a magic\
item appear nonmagical, or change the object’s aura\
so that it appears to belong to a school of magic you\
choose.\

Original layout: [[../../srd-5.2.1/pages/110|page 110]], [[../../srd-5.2.1/pages/111|page 111]].
