---
splat_schema: 1
splat_id: 7c12a608-4c5d-5a17-b3cc-5e8600636ad5
splat_kind: spell
splat_key: dnd2024:spells/alarm
splat_name: Alarm
splat_origin:
  key: dnd2024:reference/spell/alarm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 107
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: abjuration
  classes:
  - ranger
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 8 hours
  casting_time: 1 minute or Ritual
  ritual: true
  range:
    feet: 30
  material:
    description: a bell and silver wire
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/alarm
  implementation: dnd2024:spell_alarm
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 28800
    target:
      kinds:
      - door
      - window
      - area
      range_feet: 30
      maximum_cube_side_feet: 20
    exemptions:
      chooser: caster
      selection: creature_list
      timing: cast
    trigger:
      events:
      - creature.touched_area
      - creature.entered_area
      exclude_designated_creatures: true
      repeat: every_occurrence
    alarm_choice:
      chooser: caster
      timing: cast
      options:
        audible:
          sound: handbell
          seconds: 10
          audible_distance_feet: 60
          origin: warded_area
        mental:
          recipient: caster
          maximum_distance_miles: 1
          signal: mental_ping
          awakens_sleeping_recipient: true
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_alarm
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    warded_location:
      type: location
      required: true
    exempt_creatures:
      type: participant_list
      required: true
    alarm_mode:
      type: choice
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - information
  - objects
  events:
  - creature.entered_area
  - creature.touched_area
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - information
  - world_objects
  - areas
splat_conversion:
  status: specified
  category: spell
  source_sha256: a592968238f05c0b71aa445d4c7bd1434fef2f31a63a72affcef4e785c43cb67
  source_path: builtin/srd-5.2.1/spell/alarm.md
  pages:
  - 107
  engine_gaps: &id001
  - dnd2024:spell_alarm
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: eb6cb03b5a1d9c755a0550f3e899ff597a3289c09f0f29b5b3a940f696150f68
splat_clauses:
- key: source_rules
  text: '# Alarm Level 1 Abjuration (Ranger, Wizard) Casting Time: 1 minute or Ritual Range: 30 feet Components:
    V, S, M (a bell and silver wire) Duration: 8 hours You set an alarm against intrusion. Choose a door,
    a window, or an area within range that is no larger than a 20-foot Cube. Until the spell ends, an
    alarm alerts you whenever a creature touches or enters the warded area. When you cast the spell, you
    can designate creatures that won’t set off the alarm. You also choose whether the alarm is audible
    or mental: Audible Alarm. The alarm produces the sound of a handbell for 10 seconds within 60 feet
    of the warded area. Mental Alarm. You are alerted by a mental ping if you are within 1 mile of the
    warded area. This ping awakens you if you’re asleep.'
  sha256: 0ee9127d8b2075cc1e1e2a3913fa31e29e931b985d14ecdfad913508dabe4fc0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Alarm

<!-- SRD page 107 -->

Level 1 Abjuration (Ranger, Wizard)\
Casting Time: 1 minute or Ritual\
Range: 30 feet\
Components: V, S, M (a bell and silver wire)\
Duration: 8 hours\
You set an alarm against intrusion. Choose a door,\
a window, or an area within range that is no larger\
than a 20-foot Cube. Until the spell ends, an alarm\
alerts you whenever a creature touches or enters\
the warded area. When you cast the spell, you can\
designate creatures that won’t set off the alarm. You\
also choose whether the alarm is audible or mental:\
Audible Alarm. The alarm produces the sound of\
a handbell for 10 seconds within 60 feet of the\
warded area.\
Mental Alarm. You are alerted by a mental ping\
if you are within 1 mile of the warded area. This\
ping awakens you if you’re asleep.\

Original layout: [[../../srd-5.2.1/pages/107|page 107]].
