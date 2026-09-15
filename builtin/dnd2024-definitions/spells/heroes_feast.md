---
splat_schema: 1
splat_id: cd5c201b-e2f5-53a7-8358-133b1488bffa
splat_kind: spell
splat_key: dnd2024:spells/heroes_feast
splat_name: Heroes’ Feast
splat_origin:
  key: dnd2024:reference/spell/heroes_feast
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: conjuration
  classes:
  - bard
  - cleric
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: 10 minutes
  ritual: false
  range:
    mode: self
  material:
    description: a gem-encrusted bowl worth 1,000+ GP, which the spell consumes
    consumed: true
splat_conversion:
  status: partial
  category: spell
  source_sha256: 88c793314401321113f4456540d9524ec640e73f0b85f49e2932d164318b1ce5
  source_path: builtin/srd-5.2.1/spell/heroes_feast.md
  pages:
  - 140
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Heroes’ Feast Level 6 Conjuration (Bard, Cleric, Druid) Casting Time: 10 minutes Range: Self
    Components: V, S, M (a gem-encrusted bowl worth 1,000+ GP, which the spell consumes) Duration: Instantaneous
    You conjure a feast that appears on a surface in an unoccupied 10-foot Cube next to you. The feast
    takes 1 hour to consume and disappears at the end of that time, and the beneficial effects don’t set
    in until this hour is over. Up to twelve creatures can partake of the feast. A creature that partakes
    gains several benefits, which last for 24 hours. The creature has Resistance to Poison damage, and
    it has Immunity to the Frightened and Poisoned conditions. Its Hit Point maximum also increases by
    2d10, and it gains the same number of Hit Points.'
  sha256: 1aa3a4f7af27294f14292817817ec8c2d70d8a73d6dc90d78a8766d2c81c2774
  status: needs_semantic_conversion
---

# Heroes’ Feast

<!-- SRD page 140 -->

Level 6 Conjuration (Bard, Cleric, Druid)\
Casting Time: 10 minutes\
Range: Self\
Components: V, S, M (a gem-encrusted bowl worth\
1,000+ GP, which the spell consumes)\
Duration: Instantaneous\
You conjure a feast that appears on a surface in\
an unoccupied 10-foot Cube next to you. The feast\
takes 1 hour to consume and disappears at the end\
of that time, and the beneficial effects don’t set in\
until this hour is over. Up to twelve creatures can\
partake of the feast.\
A creature that partakes gains several benefits,\
which last for 24 hours. The creature has Resis-\
tance to Poison damage, and it has Immunity to the\
Frightened and Poisoned conditions. Its Hit Point\
maximum also increases by 2d10, and it gains the\
same number of Hit Points.\

Original layout: [[../../srd-5.2.1/pages/140|page 140]].
