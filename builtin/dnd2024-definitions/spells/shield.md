---
splat_schema: 1
splat_id: 33e0cba6-30a4-5de9-98df-c9152ea6c596
splat_kind: spell
splat_key: dnd2024:spells/shield
splat_name: Shield
splat_origin:
  key: dnd2024:reference/spell/shield
splat_sources:
- title: SRD 5.2.1
  locator: pp. 161, 162
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: abjuration
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 round
  casting_time: Reaction, which you take when you are hit by an attack roll or targeted by the Magic Missile
    spell
  ritual: false
  range:
    mode: self
splat_conversion:
  status: partial
  category: spell
  source_sha256: 342af40118c34eb8039b9589c1c7ea9937df0a912fa91aa7a819e6237db91e9e
  source_path: builtin/srd-5.2.1/spell/shield.md
  pages:
  - 161
  - 162
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Shield Level 1 Abjuration (Sorcerer, Wizard) Casting Time: Reaction, which you take when you
    are hit by an attack roll or targeted by the Magic Missile spell Range: Self Components: V, S Duration:
    1 round An imperceptible barrier of magical force protects you. Until the start of your next turn,
    you have a +5 bonus to AC, including against the triggering attack, and you take no damage from Magic
    Missile.'
  sha256: 8d1af2e8fecdd1712b3501c9d7662fbc92d3f6bf6ffbf48991d77f210485e5b0
  status: needs_semantic_conversion
---

# Shield

<!-- SRD page 161 -->

Level 1 Abjuration (Sorcerer, Wizard)\
Casting Time: Reaction, which you take when you are\
hit by an attack roll or targeted by the Magic Missile\
spell\
Range: Self\
Components: V, S\
Duration: 1 round\
An imperceptible barrier of magical force protects\
you. Until the start of your next turn, you have a +5\

<!-- SRD page 162 -->

bonus to AC, including against the triggering attack,\
and you take no damage from Magic Missile.\

Original layout: [[../../srd-5.2.1/pages/161|page 161]], [[../../srd-5.2.1/pages/162|page 162]].
