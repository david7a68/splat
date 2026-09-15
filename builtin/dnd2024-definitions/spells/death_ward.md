---
splat_schema: 1
splat_id: b236f1a7-8b7d-57c6-8fb5-d5df6bfb0218
splat_kind: spell
splat_key: dnd2024:spells/death_ward
splat_name: Death Ward
splat_origin:
  key: dnd2024:reference/spell/death_ward
splat_sources:
- title: SRD 5.2.1
  locator: pp. 122
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: abjuration
  classes:
  - cleric
  - paladin
  components:
  - verbal
  - somatic
  concentration: false
  duration: 8 hours
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_conversion:
  status: partial
  category: spell
  source_sha256: 6107de8f8477be99c2eb096a73b80ac43f5addce93964b01f395e99ce44a11cd
  source_path: builtin/srd-5.2.1/spell/death_ward.md
  pages:
  - 122
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Death Ward Level 4 Abjuration (Cleric, Paladin) Casting Time: Action Range: Touch Components:
    V, S Duration: 8 hours You touch a creature and grant it a measure of protection from death. The first
    time the target would drop to 0 Hit Points before the spell ends, the target instead drops to 1 Hit
    Point, and the spell ends. If the spell is still in effect when the target is subjected to an effect
    that would kill it instantly without dealing damage, that effect is negated against the target, and
    the spell ends.'
  sha256: 68320f663c57dabe516aa48e6d0f4ae7959cda5f8eb506659b67c5f7dcc9c35d
  status: needs_semantic_conversion
---

# Death Ward

<!-- SRD page 122 -->

Level 4 Abjuration (Cleric, Paladin)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: 8 hours\
You touch a creature and grant it a measure of pro-\
tection from death. The first time the target would\
drop to 0 Hit Points before the spell ends, the target\
instead drops to 1 Hit Point, and the spell ends.\
If the spell is still in effect when the target is sub-\
jected to an effect that would kill it instantly with-\
out dealing damage, that effect is negated against\
the target, and the spell ends.\

Original layout: [[../../srd-5.2.1/pages/122|page 122]].
