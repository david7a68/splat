---
splat_schema: 1
splat_id: c8f436ab-e351-503b-a543-fafe5e134c0a
splat_kind: spell
splat_key: dnd2024:spells/fly
splat_name: Fly
splat_origin:
  key: dnd2024:reference/spell/fly
splat_sources:
- title: SRD 5.2.1
  locator: pp. 133
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: transmutation
  classes:
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a feather
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: b2e91226632083570b155f07f4a142b00d20e607a80596d5d026e1e614fdb908
  source_path: builtin/srd-5.2.1/spell/fly.md
  pages:
  - 133
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Fly Level 3 Transmutation (Sorcerer, Warlock, Wizard) Casting Time: Action Range: Touch Components:
    V, S, M (a feather) Duration: Concentration, up to 10 minutes You touch a willing creature. For the
    duration, the target gains a Fly Speed of 60 feet and can hover. When the spell ends, the target falls
    if it is still aloft unless it can stop the fall. Using a Higher-Level Spell Slot. You can target
    one additional creature for each spell slot level above 3.'
  sha256: 51adef45d75ccdfaa62e0982aea61ad0edfee10bbc82250d4ddf9e1c01ecae4b
  status: needs_semantic_conversion
---

# Fly

<!-- SRD page 133 -->

Level 3 Transmutation (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a feather)\
Duration: Concentration, up to 10 minutes\
You touch a willing creature. For the duration, the\
target gains a Fly Speed of 60 feet and can hover.\
When the spell ends, the target falls if it is still aloft\
unless it can stop the fall.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 3.\

Original layout: [[../../srd-5.2.1/pages/133|page 133]].
