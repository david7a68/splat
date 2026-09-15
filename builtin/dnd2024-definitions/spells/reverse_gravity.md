---
splat_schema: 1
splat_id: ce55eb66-205d-561f-b66d-52bc60dae093
splat_kind: spell
splat_key: dnd2024:spells/reverse_gravity
splat_name: Reverse Gravity
splat_origin:
  key: dnd2024:reference/spell/reverse_gravity
splat_sources:
- title: SRD 5.2.1
  locator: pp. 159
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: transmutation
  classes:
  - druid
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
    feet: 100
  material:
    description: a lodestone and iron filings
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 0dca4f9e65aa621346cd2e7a06b3b6c83bad983fa2a8c17df71f726a3aeafa76
  source_path: builtin/srd-5.2.1/spell/reverse_gravity.md
  pages:
  - 159
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Reverse Gravity Level 7 Transmutation (Druid, Sorcerer, Wizard) Casting Time: Action Range:
    100 feet Components: V, S, M (a lodestone and iron filings) Duration: Concentration, up to 1 minute
    This spell reverses gravity in a 50-foot-radius, 100- foot high Cylinder centered on a point within
    range. All creatures and objects in that area that aren’t anchored to the ground fall upward and reach
    the top of the Cylinder. A creature can make a Dexterity saving throw to grab a fixed object it can
    reach, thus avoiding the fall upward. If a ceiling or an anchored object is encountered in this upward
    fall, creatures and objects strike it just as they would during a downward fall. If an affected creature
    or object reaches the Cylinder’s top without striking anything, it hovers there for the duration.
    When the spell ends, affected objects and creatures fall downward.'
  sha256: 2943146a6d77d89c01f5285c6186bd43d232bfcece76b7d8d6826c63c728ae72
  status: needs_semantic_conversion
---

# Reverse Gravity

<!-- SRD page 159 -->

Level 7 Transmutation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 100 feet\
Components: V, S, M (a lodestone and iron filings)\
Duration: Concentration, up to 1 minute\
This spell reverses gravity in a 50-foot-radius, 100-\
foot high Cylinder centered on a point within range.\
All creatures and objects in that area that aren’t\
anchored to the ground fall upward and reach the\
top of the Cylinder. A creature can make a Dexterity\
saving throw to grab a fixed object it can reach, thus\
avoiding the fall upward.\
If a ceiling or an anchored object is encountered\
in this upward fall, creatures and objects strike it\
just as they would during a downward fall. If an af-\
fected creature or object reaches the Cylinder’s top\
without striking anything, it hovers there for the\
duration. When the spell ends, affected objects and\
creatures fall downward.\

Original layout: [[../../srd-5.2.1/pages/159|page 159]].
