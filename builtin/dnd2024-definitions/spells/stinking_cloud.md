---
splat_schema: 1
splat_id: b4589ffa-998a-52df-bd77-d3db52107ad2
splat_kind: spell
splat_key: dnd2024:spells/stinking_cloud
splat_name: Stinking Cloud
splat_origin:
  key: dnd2024:reference/spell/stinking_cloud
splat_sources:
- title: SRD 5.2.1
  locator: pp. 165
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: conjuration
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 90
  material:
    description: a rotten egg
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 0afef82a092e679b92ee7f9d67244da700b814b41757e7b8b6ecb1cced306506
  source_path: builtin/srd-5.2.1/spell/stinking_cloud.md
  pages:
  - 165
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Stinking Cloud Level 3 Conjuration (Bard, Sorcerer, Wizard) Casting Time: Action Range: 90
    feet Components: V, S, M (a rotten egg) Duration: Concentration, up to 1 minute You create a 20-foot-radius
    Sphere of yellow, nauseating gas centered on a point within range. The cloud is Heavily Obscured.
    The cloud lingers in the air for the duration or until a strong wind (such as the one created by Gust
    of Wind) disperses it. Each creature that starts its turn in the Sphere must succeed on a Constitution
    saving throw or have the Poisoned condition until the end of the current turn. While Poisoned in this
    way, the creature can’t take an action or a Bonus Action.'
  sha256: 00b01371390d5ea2c5fe597a9309101b1eb22ed90bd748a0d80e1f042af829e4
  status: needs_semantic_conversion
---

# Stinking Cloud

<!-- SRD page 165 -->

Level 3 Conjuration (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (a rotten egg)\
Duration: Concentration, up to 1 minute\
You create a 20-foot-radius Sphere of yellow, nau-\
seating gas centered on a point within range. The\
cloud is Heavily Obscured. The cloud lingers in the\
air for the duration or until a strong wind (such as\
the one created by Gust of Wind) disperses it.\
Each creature that starts its turn in the Sphere\
must succeed on a Constitution saving throw or\
have the Poisoned condition until the end of the cur-\
rent turn. While Poisoned in this way, the creature\
can’t take an action or a Bonus Action.\

Original layout: [[../../srd-5.2.1/pages/165|page 165]].
