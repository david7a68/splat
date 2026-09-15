---
splat_schema: 1
splat_id: fa71bdec-f747-5cbb-a32b-e357056938af
splat_kind: spell
splat_key: dnd2024:spells/sacred_flame
splat_name: Sacred Flame
splat_origin:
  key: dnd2024:reference/spell/sacred_flame
splat_sources:
- title: SRD 5.2.1
  locator: pp. 159
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: evocation
  classes:
  - cleric
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/sacred_flame
  implementation: dnd2024:spell_sacred_flame
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      visible: true
    save:
      ability: dexterity
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
            sides: 8
            add: 0
          type: radiant
      success:
        effect: none
    ignore_cover_for_save:
    - half
    - three_quarters
splat_contracts:
- id: dnd2024:spell_sacred_flame
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
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: c1aabe1b8c1cf3413915cf38501d4652d3fdfd0702a9e652d835958d9c1694a5
  source_path: builtin/srd-5.2.1/spell/sacred_flame.md
  pages:
  - 159
  engine_gaps: &id001
  - dnd2024:spell_sacred_flame
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 80c0a1898861b5e97025585e7f1543eea5fa944f18675945cf7e8eb9a72eda7c
splat_clauses:
- key: source_rules
  text: '# Sacred Flame Evocation Cantrip (Cleric) Casting Time: Action Range: 60 feet Components: V,
    S Duration: Instantaneous Flame-like radiance descends on a creature that you can see within range.
    The target must succeed on a Dexterity saving throw or take 1d8 Radiant damage. The target gains no
    benefit from Half Cover or Three-Quarters Cover for this save. Cantrip Upgrade. The damage increases
    by 1d8 when you reach levels 5 (2d8), 11 (3d8), and 17 (4d8).'
  sha256: 4d7ad905c8c268e6f4dea8ce6c53e346172c4df56cda5e25f8df1a9ddbc27cf1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sacred Flame

<!-- SRD page 159 -->

Evocation Cantrip (Cleric)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
Flame-like radiance descends on a creature that you\
can see within range. The target must succeed on\
a Dexterity saving throw or take 1d8 Radiant dam-\
age. The target gains no benefit from Half Cover or\
Three-Quarters Cover for this save.\
Cantrip Upgrade. The damage increases by 1d8\
when you reach levels 5 (2d8), 11 (3d8), and 17\
(4d8).\

Original layout: [[../../srd-5.2.1/pages/159|page 159]].
