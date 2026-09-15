---
splat_schema: 1
splat_id: 0c3a1349-5ee0-5340-a9ab-d57829167f14
splat_kind: spell
splat_key: dnd2024:spells/barkskin
splat_name: Barkskin
splat_origin:
  key: dnd2024:reference/spell/barkskin
splat_sources:
- title: SRD 5.2.1
  locator: pp. 112
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - druid
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Bonus Action
  ritual: false
  range:
    mode: touch
  material:
    description: a handful of bark
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: ad1506d11647507a532bab927b7e743c22340a286a37b8d551885e89046cb81c
  source_path: builtin/srd-5.2.1/spell/barkskin.md
  pages:
  - 112
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Barkskin Level 2 Transmutation (Druid, Ranger) Casting Time: Bonus Action Range: Touch Component:
    V, S, M (a handful of bark) Duration: 1 hour You touch a willing creature. Until the spell ends, the
    target’s skin assumes a bark-like appearance, and the target has an Armor Class of 17 if its AC is
    lower than that.'
  sha256: f7cfdeda94759a48f4a523851d41a5a51fae49a14f7374dc7827e1351f1b9119
  status: needs_semantic_conversion
---

# Barkskin

<!-- SRD page 112 -->

Level 2 Transmutation (Druid, Ranger)\
Casting Time: Bonus Action\
Range: Touch\
Component: V, S, M (a handful of bark)\
Duration: 1 hour\
You touch a willing creature. Until the spell ends,\
the target’s skin assumes a bark-like appearance,\
and the target has an Armor Class of 17 if its AC is\
lower than that.\

Original layout: [[../../srd-5.2.1/pages/112|page 112]].
