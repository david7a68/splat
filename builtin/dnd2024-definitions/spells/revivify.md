---
splat_schema: 1
splat_id: a5e28636-7477-5376-a7d6-7a91d1928c65
splat_kind: spell
splat_key: dnd2024:spells/revivify
splat_name: Revivify
splat_origin:
  key: dnd2024:reference/spell/revivify
splat_sources:
- title: SRD 5.2.1
  locator: pp. 159
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: necromancy
  classes:
  - cleric
  - druid
  - paladin
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a diamond worth 300+ GP, which the spell consumes
    consumed: true
splat_conversion:
  status: partial
  category: spell
  source_sha256: 5e4d2372d37c0675e0ec438f19ec47802c27d9011cf3930c5f2b4d4ca3c07479
  source_path: builtin/srd-5.2.1/spell/revivify.md
  pages:
  - 159
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Revivify Level 3 Necromancy (Cleric, Druid, Paladin, Ranger) Casting Time: Action Range: Touch
    Components: V, S, M (a diamond worth 300+ GP, which the spell consumes) Duration: Instantaneous You
    touch a creature that has died within the last minute. That creature revives with 1 Hit Point. This
    spell can’t revive a creature that has died of old age, nor does it restore any missing body parts.'
  sha256: 44f297ca5a8a97ce4a3724ade49af367dff03f2b64b4b963864aec6ff9a0b0f9
  status: needs_semantic_conversion
---

# Revivify

<!-- SRD page 159 -->

Level 3 Necromancy (Cleric, Druid, Paladin, Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a diamond worth 300+ GP, which\
the spell consumes)\
Duration: Instantaneous\
You touch a creature that has died within the last\
minute. That creature revives with 1 Hit Point. This\
spell can’t revive a creature that has died of old age,\
nor does it restore any missing body parts.\

Original layout: [[../../srd-5.2.1/pages/159|page 159]].
