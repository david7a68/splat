---
splat_schema: 1
splat_id: 5eeefa95-08af-588f-bed1-c359739b95ce
splat_kind: spell
splat_key: dnd2024:spells/faerie_fire
splat_name: Faerie Fire
splat_origin:
  key: dnd2024:reference/spell/faerie_fire
splat_sources:
- title: SRD 5.2.1
  locator: pp. 129
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: evocation
  classes:
  - bard
  - druid
  components:
  - verbal
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_conversion:
  status: partial
  category: spell
  source_sha256: c1cf5a11ac10c982047c847d9f879ce33534bd8c643de7630ffa377051a772ad
  source_path: builtin/srd-5.2.1/spell/faerie_fire.md
  pages:
  - 129
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Faerie Fire Level 1 Evocation (Bard, Druid) Casting Time: Action Range: 60 feet Components:
    V Duration: Concentration, up to 1 minute Objects in a 20-foot Cube within range are outlined in blue,
    green, or violet light (your choice). Each creature in the Cube is also outlined if it fails a Dexterity
    saving throw. For the duration, objects and affected creatures shed Dim Light in a 10-foot radius
    and can’t benefit from the Invisible condition. Attack rolls against an affected creature or object
    have Advantage if the attacker can see it.'
  sha256: 3251bcf26312351bba6c9aedacf405080bc3eb11dcce3b7a3669ab700e68d507
  status: needs_semantic_conversion
---

# Faerie Fire

<!-- SRD page 129 -->

Level 1 Evocation (Bard, Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Concentration, up to 1 minute\
Objects in a 20-foot Cube within range are outlined\
in blue, green, or violet light (your choice). Each\
creature in the Cube is also outlined if it fails a Dex-\
terity saving throw. For the duration, objects and\
affected creatures shed Dim Light in a 10-foot ra-\
dius and can’t benefit from the Invisible condition.\
Attack rolls against an affected creature or object\
have Advantage if the attacker can see it.\

Original layout: [[../../srd-5.2.1/pages/129|page 129]].
