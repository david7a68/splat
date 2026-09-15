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
splat_rule:
  mechanic: dnd2024:spells/pass_without_trace
  implementation: dnd2024:spell_pass_without_trace
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 3600
    area:
      shape: emanation
      radius_feet: 30
      origin: caster
      moves_with_origin: true
    beneficiaries:
      caster: true
      additional_creatures: caster_choice
      requires: in_aura
    benefits:
      dexterity_stealth_check_bonus: 10
      leave_tracks: false
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_pass_without_trace
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    beneficiaries:
      type: participant_list
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - movement
  - areas
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: d48cd1030253036df7aaf318010c898e69f408f8774c56b3af2a5d80cc21d506
  source_path: builtin/srd-5.2.1/spell/pass_without_trace.md
  pages:
  - 151
  engine_gaps: &id001
  - dnd2024:spell_pass_without_trace
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: b5b385f640d9e21df71d5028d97160da9f9eacc1bd9eaec9198888d42df5f3fb
splat_clauses:
- key: source_rules
  text: '# Pass without Trace Level 2 Abjuration (Druid, Ranger) Casting Time: Action Range: Self Components:
    V, S, M (ashes from burned mistletoe) Duration: Concentration, up to 1 hour You radiate a concealing
    aura in a 30-foot Emanation for the duration. While in the aura, you and each creature you choose
    have a +10 bonus to Dexterity (Stealth) checks and leave no tracks.'
  sha256: 7ef64ebeeb4a98e1e7bb1761745ee0ae33bb6ee8055d044897743edfe5c93989
  status: contract_specified
splat_requires_mechanics: *id001
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
