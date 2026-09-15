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
splat_conversion:
  status: partial
  category: spell
  source_sha256: a592968238f05c0b71aa445d4c7bd1434fef2f31a63a72affcef4e785c43cb67
  source_path: builtin/srd-5.2.1/spell/alarm.md
  pages:
  - 107
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
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
  status: needs_semantic_conversion
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
