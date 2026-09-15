---
splat_schema: 1
splat_id: 8938815b-ac6e-599b-a3f5-60cbc66a3bc6
splat_kind: spell
splat_key: dnd2024:spells/gentle_repose
splat_name: Gentle Repose
splat_origin:
  key: dnd2024:reference/spell/gentle_repose
splat_sources:
- title: SRD 5.2.1
  locator: pp. 135
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: necromancy
  classes:
  - cleric
  - paladin
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 10 days
  casting_time: Action or Ritual
  ritual: true
  range:
    mode: touch
  material:
    description: 2 Copper Pieces, which the spell consumes
    consumed: true
splat_conversion:
  status: partial
  category: spell
  source_sha256: d745216fc7956fcda4fdf1c2a3ad92c3fafbdc73a7e5d9ae91dbf5fe39abb869
  source_path: builtin/srd-5.2.1/spell/gentle_repose.md
  pages:
  - 135
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Gentle Repose Level 2 Necromancy (Cleric, Paladin, Wizard) Casting Time: Action or Ritual Range:
    Touch Components: V, S, M (2 Copper Pieces, which the spell consumes) Duration: 10 days You touch
    a corpse or other remains. For the duration, the target is protected from decay and can’t become Undead.
    The spell also effectively extends the time limit on raising the target from the dead, since days
    spent under the influence of this spell don’t count against the time limit of spells such as Raise
    Dead.'
  sha256: 860dae23981fa37678427e2f7271d8f40c51633345081f46583ff95c1360cb2f
  status: needs_semantic_conversion
---

# Gentle Repose

<!-- SRD page 135 -->

Level 2 Necromancy (Cleric, Paladin, Wizard)\
Casting Time: Action or Ritual\
Range: Touch\
Components: V, S, M (2 Copper Pieces, which the spell\
consumes)\
Duration: 10 days\
You touch a corpse or other remains. For the dura-\
tion, the target is protected from decay and can’t\
become Undead.\
The spell also effectively extends the time limit on\
raising the target from the dead, since days spent\
under the influence of this spell don’t count against\
the time limit of spells such as Raise Dead.\

Original layout: [[../../srd-5.2.1/pages/135|page 135]].
