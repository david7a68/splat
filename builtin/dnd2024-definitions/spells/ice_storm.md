---
splat_schema: 1
splat_id: fbc7531b-6863-52d4-a864-6056ce2201b1
splat_kind: spell
splat_key: dnd2024:spells/ice_storm
splat_name: Ice Storm
splat_origin:
  key: dnd2024:reference/spell/ice_storm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 141, 142
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: evocation
  classes:
  - druid
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 300
  material:
    description: a mitten
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 76f296a109811b0bda67e55316d9db20bca86ee070d26b6b8a7812de62afcf92
  source_path: builtin/srd-5.2.1/spell/ice_storm.md
  pages:
  - 141
  - 142
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Ice Storm Level 4 Evocation (Druid, Sorcerer, Wizard) Casting Time: Action Range: 300 feet
    Components: V, S, M (a mitten) Duration: Instantaneous Hail falls in a 20-foot-radius, 40-foot-high
    Cylinder centered on a point within range. Each creature in the Cylinder makes a Dexterity saving
    throw. A creature takes 2d10 Bludgeoning damage and 4d6 Cold damage on a failed save or half as much
    damage on a successful one. Hailstones turn ground in the Cylinder into Difficult Terrain until the
    end of your next turn. Using a Higher-Level Spell Slot. The Bludgeoning damage increases by 1d10 for
    each spell slot level above 4.'
  sha256: d952f8fb0f19a17772d81603d51b540e59ed809d040479bc3506acf47fb6bdff
  status: needs_semantic_conversion
---

# Ice Storm

<!-- SRD page 141 -->

Level 4 Evocation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 300 feet\
Components: V, S, M (a mitten)\
Duration: Instantaneous\
Hail falls in a 20-foot-radius, 40-foot-high Cylinder\
centered on a point within range. Each creature\
in the Cylinder makes a Dexterity saving throw. A\
creature takes 2d10 Bludgeoning damage and 4d6\
Cold damage on a failed save or half as much dam-\
age on a successful one.\
Hailstones turn ground in the Cylinder into Diffi-\
cult Terrain until the end of your next turn.\

<!-- SRD page 142 -->

Using a Higher-Level Spell Slot. The Bludgeoning\
damage increases by 1d10 for each spell slot level\
above 4.\

Original layout: [[../../srd-5.2.1/pages/141|page 141]], [[../../srd-5.2.1/pages/142|page 142]].
