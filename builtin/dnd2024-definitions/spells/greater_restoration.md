---
splat_schema: 1
splat_id: 55c81461-36e7-5f03-97ce-b861b2cdc156
splat_kind: spell
splat_key: dnd2024:spells/greater_restoration
splat_name: Greater Restoration
splat_origin:
  key: dnd2024:reference/spell/greater_restoration
splat_sources:
- title: SRD 5.2.1
  locator: pp. 137
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: abjuration
  classes:
  - bard
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
    description: diamond dust worth 100+ GP, which the spell consumes
    consumed: true
splat_conversion:
  status: partial
  category: spell
  source_sha256: 97870f1e4c512922694b1885f47dcd7280632c3a4fcdc900a5382f8280c7ed1f
  source_path: builtin/srd-5.2.1/spell/greater_restoration.md
  pages:
  - 137
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Greater Restoration Level 5 Abjuration (Bard, Cleric, Druid, Paladin, Ranger) Casting Time:
    Action Range: Touch Components: V, S, M (diamond dust worth 100+ GP, which the spell consumes) Duration:
    Instantaneous You touch a creature and magically remove one of the following effects from it: • 1
    Exhaustion level • The Charmed or Petrified condition • A curse, including the target’s Attunement
    to a cursed magic item • Any reduction to one of the target’s ability scores • Any reduction to the
    target’s Hit Point maximum'
  sha256: b7e75e923f34ae355c91950e5f5b27d3877c165e4aa26deb2d589c0e7a0023e4
  status: needs_semantic_conversion
---

# Greater Restoration

<!-- SRD page 137 -->

Level 5 Abjuration (Bard, Cleric, Druid, Paladin,\
Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (diamond dust worth 100+ GP,\
which the spell consumes)\
Duration: Instantaneous\
You touch a creature and magically remove one of\
the following effects from it:\
•  1 Exhaustion level\
•  The Charmed or Petrified condition\
•  A curse, including the target’s Attunement to a\
cursed magic item\
•  Any reduction to one of the target’s ability scores\
•  Any reduction to the target’s Hit Point maximum\

Original layout: [[../../srd-5.2.1/pages/137|page 137]].
