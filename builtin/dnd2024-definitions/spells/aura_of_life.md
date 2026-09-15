---
splat_schema: 1
splat_id: d3530c90-dbc4-5def-bda2-896fc7190e55
splat_kind: spell
splat_key: dnd2024:spells/aura_of_life
splat_name: Aura of Life
splat_origin:
  key: dnd2024:reference/spell/aura_of_life
splat_sources:
- title: SRD 5.2.1
  locator: pp. 111
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: abjuration
  classes:
  - cleric
  - paladin
  components:
  - verbal
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_conversion:
  status: partial
  category: spell
  source_sha256: 1559da6db8e15139feafe9f4d283255e951605f88fc571cd69a89d2e4f362769
  source_path: builtin/srd-5.2.1/spell/aura_of_life.md
  pages:
  - 111
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Aura of Life Level 4 Abjuration (Cleric, Paladin) Casting Time: Action Range: Self Components:
    V Duration: Concentration, up to 10 minutes An aura radiates from you in a 30-foot Emanation for the
    duration. While in the aura, you and your allies have Resistance to Necrotic damage, and your Hit
    Point maximums can’t be reduced. If an ally with 0 Hit Points starts its turn in the aura, that ally
    regains 1 Hit Point.'
  sha256: 9794da94c54fef3e425e92102bbe7367930512a47438d375db49f34368582ec5
  status: needs_semantic_conversion
---

# Aura of Life

<!-- SRD page 111 -->

Level 4 Abjuration (Cleric, Paladin)\
Casting Time: Action\
Range: Self\
Components: V\
Duration: Concentration, up to 10 minutes\
An aura radiates from you in a 30-foot Emanation\
for the duration. While in the aura, you and your\
allies have Resistance to Necrotic damage, and your\
Hit Point maximums can’t be reduced. If an ally with\
0 Hit Points starts its turn in the aura, that ally re-\
gains 1 Hit Point.\

Original layout: [[../../srd-5.2.1/pages/111|page 111]].
