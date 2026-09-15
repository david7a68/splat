---
splat_schema: 1
splat_id: d533b3c2-46d0-5370-b015-b543d0fa3e9e
splat_kind: spell
splat_key: dnd2024:spells/starry_wisp
splat_name: Starry Wisp
splat_origin:
  key: dnd2024:reference/spell/starry_wisp
splat_sources:
- title: SRD 5.2.1
  locator: pp. 165
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: evocation
  classes:
  - bard
  - druid
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
  mechanic: dnd2024:spells/starry_wisp
  implementation: dnd2024:spell_starry_wisp
  parameters:
    targets:
      kind: creature_or_object
      count: 1
      within_spell_range: true
    attack:
      kind: ranged_spell
      hit:
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
        light:
          level: dim
          radius_feet: 10
        cannot_benefit_from_condition: invisible
        until: caster_next_turn_end
splat_contracts:
- id: dnd2024:spell_starry_wisp
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
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
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
  - attacks
  events:
  - spell.cast_completed
  - attack.roll
  - attack.hit
  - movement.completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - attacks
  - spatial_relationships
  - conditions
splat_conversion:
  status: specified
  category: spell
  source_sha256: 123617237c7e7a88236e33e0bcc451904438d38523a1d47eb5f55971a3fa4e6a
  source_path: builtin/srd-5.2.1/spell/starry_wisp.md
  pages:
  - 165
  engine_gaps: &id001
  - dnd2024:spell_starry_wisp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: c9959a345a45e4e9efc560f4dd5199c54341342a058912dadc7fd81162a3e267
splat_clauses:
- key: source_rules
  text: '# Starry Wisp Evocation Cantrip (Bard, Druid) Casting Time: Action Range: 60 feet Components:
    V, S Duration: Instantaneous You launch a mote of light at one creature or object within range. Make
    a ranged spell attack against the target. On a hit, the target takes 1d8 Radiant damage, and until
    the end of your next turn, it emits Dim Light in a 10-foot radius and can’t benefit from the Invisible
    condition. Cantrip Upgrade. The damage increases by 1d8 when you reach levels 5 (2d8), 11 (3d8), and
    17 (4d8).'
  sha256: 06816c0a52648680b08b05c3d0fe85cf92ec64be27e925e961564dd219fabfb1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Starry Wisp

<!-- SRD page 165 -->

Evocation Cantrip (Bard, Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
You launch a mote of light at one creature or object\
within range. Make a ranged spell attack against the\
target. On a hit, the target takes 1d8 Radiant dam-\
age, and until the end of your next turn, it emits Dim\
Light in a 10-foot radius and can’t benefit from the\
Invisible condition.\
Cantrip Upgrade. The damage increases by 1d8\
when you reach levels 5 (2d8), 11 (3d8), and 17\
(4d8).\

Original layout: [[../../srd-5.2.1/pages/165|page 165]].
