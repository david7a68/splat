---
splat_schema: 1
splat_id: 962403d3-4ca7-58f0-a9f5-debe7fdb4788
splat_kind: spell
splat_key: dnd2024:spells/identify
splat_name: Identify
splat_origin:
  key: dnd2024:reference/spell/identify
splat_sources:
- title: SRD 5.2.1
  locator: pp. 142
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: divination
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: 1 minute or Ritual
  ritual: true
  range:
    mode: touch
  material:
    description: a pearl worth 100+ GP
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 2e82430c4b3e2a484c9ec43a1c07b995119877d6a842e2bc203c044632072c1c
  source_path: builtin/srd-5.2.1/spell/identify.md
  pages:
  - 142
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Identify Level 1 Divination (Bard, Wizard) Casting Time: 1 minute or Ritual Range: Touch Components:
    V, S, M (a pearl worth 100+ GP) Duration: Instantaneous You touch an object throughout the spell’s
    casting. If the object is a magic item or some other magical object, you learn its properties and
    how to use them, whether it requires Attunement, and how many charges it has, if any. You learn whether
    any ongoing spells are affecting the item and what they are. If the item was created by a spell, you
    learn that spell’s name. If you instead touch a creature throughout the casting, you learn which ongoing
    spells, if any, are currently affecting it.'
  sha256: 1fbab574c49e121f6d313a743fc77419c85969df8220b33d975cdf29dd92bfe0
  status: needs_semantic_conversion
---

# Identify

<!-- SRD page 142 -->

Level 1 Divination (Bard, Wizard)\
Casting Time: 1 minute or Ritual\
Range: Touch\
Components: V, S, M (a pearl worth 100+ GP)\
Duration: Instantaneous\
You touch an object throughout the spell’s casting.\
If the object is a magic item or some other magi-\
cal object, you learn its properties and how to use\
them, whether it requires Attunement, and how\
many charges it has, if any. You learn whether any\
ongoing spells are affecting the item and what they\
are. If the item was created by a spell, you learn that\
spell’s name.\
If you instead touch a creature throughout the\
casting, you learn which ongoing spells, if any, are\
currently affecting it.\

Original layout: [[../../srd-5.2.1/pages/142|page 142]].
