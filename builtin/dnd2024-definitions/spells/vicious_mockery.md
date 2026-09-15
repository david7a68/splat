---
splat_schema: 1
splat_id: aa37a87f-10b2-5eae-b22b-db3e9e858235
splat_kind: spell
splat_key: dnd2024:spells/vicious_mockery
splat_name: Vicious Mockery
splat_origin:
  key: dnd2024:reference/spell/vicious_mockery
splat_sources:
- title: SRD 5.2.1
  locator: pp. 171, 172
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: enchantment
  classes:
  - bard
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/vicious_mockery
  implementation: dnd2024:spell_vicious_mockery
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      perception_any:
      - caster_sees_target
      - caster_hears_target
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        damage:
          dice:
            count:
              by_caster_level:
              - minimum: 1
                count: 1
              - minimum: 5
                count: 2
              - minimum: 11
                count: 3
              - minimum: 17
                count: 4
            sides: 6
            add: 0
          type: psychic
        disadvantage_next_attack: true
        until: target_next_turn_end
      success:
        effect: none
splat_contracts:
- id: dnd2024:spell_vicious_mockery
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
    save_dc:
      type: integer
      required: true
    caster_level:
      type: integer
      required: true
    choices:
      required: false
      type: choice
    trigger:
      required: false
      type: event
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  - attack.roll
  - attack.hit
  - turn.end
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - saving_throws
  - roll_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: a7a7f4834b98596fee0b0c71dec13bba208de136760a0ce358829838791b2a09
  source_path: builtin/srd-5.2.1/spell/vicious_mockery.md
  pages:
  - 171
  - 172
  engine_gaps: &id001
  - dnd2024:spell_vicious_mockery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: fded6f8cf3e5fc06abf6e52f074b576c701a1a8bf1ae699b707d80146681c1c3
splat_clauses:
- key: source_rules
  text: '# Vicious Mockery Enchantment Cantrip (Bard) Casting Time: Action Range: 60 feet Components:
    V Duration: Instantaneous You unleash a string of insults laced with subtle enchantments at one creature
    you can see or hear within range. The target must succeed on a Wisdom saving throw or take 1d6 Psychic
    damage and have Disadvantage on the next attack roll it makes before the end of its next turn. Cantrip
    Upgrade. The damage increases by 1d6 when you reach levels 5 (2d6), 11 (3d6), and 17 (4d6).'
  sha256: 915fda0b5d72c8d43943dd8ee82709b3074b68f99d43769c705af98a775223ca
  status: contract_specified
splat_requires_mechanics: *id001
---

# Vicious Mockery

<!-- SRD page 171 -->

Enchantment Cantrip (Bard)\
Casting Time: Action\
Range: 60 feet\

<!-- SRD page 172 -->

Components: V\
Duration: Instantaneous\
You unleash a string of insults laced with subtle\
enchantments at one creature you can see or hear\
within range. The target must succeed on a Wisdom\
saving throw or take 1d6 Psychic damage and have\
Disadvantage on the next attack roll it makes before\
the end of its next turn.\
Cantrip Upgrade. The damage increases by 1d6\
when you reach levels 5 (2d6), 11 (3d6), and 17\
(4d6).\

Original layout: [[../../srd-5.2.1/pages/171|page 171]], [[../../srd-5.2.1/pages/172|page 172]].
