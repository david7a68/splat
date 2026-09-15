---
splat_schema: 1
splat_id: 5cf2e046-608a-5241-aebe-1cc8b112d6e6
splat_kind: spell
splat_key: dnd2024:spells/clone
splat_name: Clone
splat_origin:
  key: dnd2024:reference/spell/clone
splat_sources:
- title: SRD 5.2.1
  locator: pp. 115, 116
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: necromancy
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: 1 hour
  ritual: false
  range:
    mode: touch
  material:
    description: a diamond worth 1,000+ GP, which the spell consumes, and a sealable vessel worth 2,000+
      GP that is large enough to hold the creature being cloned
    consumed: true
splat_conversion:
  status: partial
  category: spell
  source_sha256: a55c4c417e31a04ae46b34297c5a2e23487e270841cfe92c3ec5b461d14b330b
  source_path: builtin/srd-5.2.1/spell/clone.md
  pages:
  - 115
  - 116
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Clone Level 8 Necromancy (Wizard) Casting Time: 1 hour Range: Touch Components: V, S, M (a
    diamond worth 1,000+ GP, which the spell consumes, and a sealable vessel worth 2,000+ GP that is large
    enough to hold the creature being cloned) Duration: Instantaneous You touch a creature or at least
    1 cubic inch of its flesh. An inert duplicate of that creature forms inside the vessel used in the
    spell’s casting and finishes growing after 120 days; you choose whether the finished clone is the
    same age as the creature or younger. The clone remains inert and endures indefinitely while its vessel
    remains undisturbed. If the original creature dies after the clone finishes forming, the creature’s
    soul transfers to the clone if the soul is free and willing to return. The clone is physically identical
    to the original and has the same personality, memories, and abilities, but none of the original’s
    equipment. The creature’s original remains, if any, become inert and can’t be revived, since the creature’s
    soul is elsewhere.'
  sha256: 793bab75e11ae7b622b564c73292638185a1055f897c191275be0666b6fefab9
  status: needs_semantic_conversion
---

# Clone

<!-- SRD page 115 -->

Level 8 Necromancy (Wizard)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (a diamond worth 1,000+ GP,\
which the spell consumes, and a sealable vessel worth\
2,000+ GP that is large enough to hold the creature\
being cloned)\
Duration: Instantaneous\

<!-- SRD page 116 -->

You touch a creature or at least 1 cubic inch of its\
flesh. An inert duplicate of that creature forms\
inside the vessel used in the spell’s casting and fin-\
ishes growing after 120 days; you choose whether\
the finished clone is the same age as the creature or\
younger. The clone remains inert and endures indef-\
initely while its vessel remains undisturbed.\
If the original creature dies after the clone fin-\
ishes forming, the creature’s soul transfers to the\
clone if the soul is free and willing to return. The\
clone is physically identical to the original and has\
the same personality, memories, and abilities, but\
none of the original’s equipment. The creature’s\
original remains, if any, become inert and can’t be\
revived, since the creature’s soul is elsewhere.\

Original layout: [[../../srd-5.2.1/pages/115|page 115]], [[../../srd-5.2.1/pages/116|page 116]].
