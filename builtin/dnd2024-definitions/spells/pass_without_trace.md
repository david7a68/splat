---
splat_schema: 1
splat_id: 463d14d1-fd72-5ea7-9eb9-51797f64ecee
splat_kind: spell
splat_key: dnd2024:spells/pass_without_trace
splat_name: Pass without Trace
splat_origin:
  key: dnd2024:reference/spell/pass_without_trace
splat_sources:
- title: SRD 5.2.1
  locator: pp. 151
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: abjuration
  classes:
  - druid
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: ashes from burned mistletoe
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: d48cd1030253036df7aaf318010c898e69f408f8774c56b3af2a5d80cc21d506
  source_path: builtin/srd-5.2.1/spell/pass_without_trace.md
  pages:
  - 151
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Pass without Trace Level 2 Abjuration (Druid, Ranger) Casting Time: Action Range: Self Components:
    V, S, M (ashes from burned mistletoe) Duration: Concentration, up to 1 hour You radiate a concealing
    aura in a 30-foot Emanation for the duration. While in the aura, you and each creature you choose
    have a +10 bonus to Dexterity (Stealth) checks and leave no tracks.'
  sha256: 7ef64ebeeb4a98e1e7bb1761745ee0ae33bb6ee8055d044897743edfe5c93989
  status: needs_semantic_conversion
---

# Pass without Trace

<!-- SRD page 151 -->

Level 2 Abjuration (Druid, Ranger)\
Casting Time: Action\
Range: Self\
Components: V, S, M (ashes from burned mistletoe)\
Duration: Concentration, up to 1 hour\
You radiate a concealing aura in a 30-foot Emana-\
tion for the duration. While in the aura, you and\
each creature you choose have a +10 bonus to Dex-\
terity (Stealth) checks and leave no tracks.\

Original layout: [[../../srd-5.2.1/pages/151|page 151]].
