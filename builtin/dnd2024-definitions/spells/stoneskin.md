---
splat_schema: 1
splat_id: 5c5919d9-67ca-523e-9a4d-d49534a1c746
splat_kind: spell
splat_key: dnd2024:spells/stoneskin
splat_name: Stoneskin
splat_origin:
  key: dnd2024:reference/spell/stoneskin
splat_sources:
- title: SRD 5.2.1
  locator: pp. 165
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: transmutation
  classes:
  - druid
  - ranger
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: diamond dust worth 100+ GP, which the spell consumes
    consumed: true
splat_conversion:
  status: partial
  category: spell
  source_sha256: 34e4615d3bc9dd2d9ef88f86630ca09ecc7cec93b89819e105e41f2324a739ff
  source_path: builtin/srd-5.2.1/spell/stoneskin.md
  pages:
  - 165
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Stoneskin Level 4 Transmutation (Druid, Ranger, Sorcerer, Wizard) Casting Time: Action Range:
    Touch Components: V, S, M (diamond dust worth 100+ GP, which the spell consumes) Duration: Concentration,
    up to 1 hour Until the spell ends, one willing creature you touch has Resistance to Bludgeoning, Piercing,
    and Slashing damage.'
  sha256: 023e5b5a47732b661a2f5747da45a9b37648a6e3e18482910c990ee4e99a458b
  status: needs_semantic_conversion
---

# Stoneskin

<!-- SRD page 165 -->

Level 4 Transmutation (Druid, Ranger, Sorcerer,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (diamond dust worth 100+ GP,\
which the spell consumes)\
Duration: Concentration, up to 1 hour\
Until the spell ends, one willing creature you touch\
has Resistance to Bludgeoning, Piercing, and Slash-\
ing damage.\

Original layout: [[../../srd-5.2.1/pages/165|page 165]].
