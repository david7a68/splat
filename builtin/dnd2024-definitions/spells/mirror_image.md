---
splat_schema: 1
splat_id: c53c534a-c02f-5da4-a876-46a1d9d6b4b7
splat_kind: spell
splat_key: dnd2024:spells/mirror_image
splat_name: Mirror Image
splat_origin:
  key: dnd2024:reference/spell/mirror_image
splat_sources:
- title: SRD 5.2.1
  locator: pp. 150
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: illusion
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_conversion:
  status: partial
  category: spell
  source_sha256: 595c87eb9779604567054b62607d85e986ada9de63976fb28aa6f37d11417a40
  source_path: builtin/srd-5.2.1/spell/mirror_image.md
  pages:
  - 150
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Mirror Image Level 2 Illusion (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    Self Components: V, S Duration: 1 minute Three illusory duplicates of yourself appear in your space.
    Until the spell ends, the duplicates move with you and mimic your actions, shifting position so it’s
    impossible to track which image is real. Each time a creature hits you with an attack roll during
    the spell’s duration, roll a d6 for each of your remaining duplicates. If any of the d6s rolls a 3
    or higher, one of the duplicates is hit instead of you, and the duplicate is destroyed. The duplicates
    otherwise ignore all other damage and effects. The spell ends when all three duplicates are destroyed.
    A creature is unaffected by this spell if it has the Blinded condition, Blindsight, or Truesight.'
  sha256: 51c6e5d6f9a8d937df6a90b5b573929bcb11ca5dc27add0dc4bf8238f344cc97
  status: needs_semantic_conversion
---

# Mirror Image

<!-- SRD page 150 -->

Level 2 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: 1 minute\
Three illusory duplicates of yourself appear in your\
space. Until the spell ends, the duplicates move with\
you and mimic your actions, shifting position so it’s\
impossible to track which image is real.\
Each time a creature hits you with an attack roll\
during the spell’s duration, roll a d6 for each of\
your remaining duplicates. If any of the d6s rolls\
a 3 or higher, one of the duplicates is hit instead of\
you, and the duplicate is destroyed. The duplicates\
otherwise ignore all other damage and effects. The\
spell ends when all three duplicates are destroyed.\
A creature is unaffected by this spell if it has the\
Blinded condition, Blindsight, or Truesight.\

Original layout: [[../../srd-5.2.1/pages/150|page 150]].
