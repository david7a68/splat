---
splat_schema: 1
splat_id: 7e9bfc16-0d2f-55b1-87a6-d440c7271add
splat_kind: spell
splat_key: dnd2024:spells/water_breathing
splat_name: Water Breathing
splat_origin:
  key: dnd2024:reference/spell/water_breathing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 173
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: transmutation
  classes:
  - druid
  - ranger
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 24 hours
  casting_time: Action or Ritual
  ritual: true
  range:
    feet: 30
  material:
    description: a short reed
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 05922fd9adb7848e200d1ddc447d4510e888b7747f8695ae854bcdc6beedb754
  source_path: builtin/srd-5.2.1/spell/water_breathing.md
  pages:
  - 173
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Water Breathing Level 3 Transmutation (Druid, Ranger, Sorcerer, Wizard) Casting Time: Action
    or Ritual Range: 30 feet Components: V, S, M (a short reed) Duration: 24 hours This spell grants up
    to ten willing creatures of your choice within range the ability to breathe underwater until the spell
    ends. Affected creatures also retain their normal mode of respiration.'
  sha256: 26a86b2c131737ca0420967e5489fc8c2ddffdfc2f8349bd6f5abc77fdbbc09a
  status: needs_semantic_conversion
---

# Water Breathing

<!-- SRD page 173 -->

Level 3 Transmutation (Druid, Ranger, Sorcerer,\
Wizard)\
Casting Time: Action or Ritual\
Range: 30 feet\
Components: V, S, M (a short reed)\
Duration: 24 hours\
This spell grants up to ten willing creatures of your\
choice within range the ability to breathe under-\
water until the spell ends. Affected creatures also\
retain their normal mode of respiration.\

Original layout: [[../../srd-5.2.1/pages/173|page 173]].
