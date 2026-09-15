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
splat_rule:
  mechanic: dnd2024:spells/stoneskin
  implementation: dnd2024:spell_stoneskin
  parameters:
    targeting:
      kind: creature
      count: 1
      willing: true
      touch: true
    resistance:
      types:
      - bludgeoning
      - piercing
      - slashing
      duration: spell_duration
      magical_damage_included: true
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_stoneskin
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
splat_conversion:
  status: specified
  category: spell
  source_sha256: 34e4615d3bc9dd2d9ef88f86630ca09ecc7cec93b89819e105e41f2324a739ff
  source_path: builtin/srd-5.2.1/spell/stoneskin.md
  pages:
  - 165
  engine_gaps: &id001
  - dnd2024:spell_stoneskin
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 2126d5567f6d87ac31f9553525626811cd369167418fd517419047c7cb71087d
splat_clauses:
- key: source_rules
  text: '# Stoneskin Level 4 Transmutation (Druid, Ranger, Sorcerer, Wizard) Casting Time: Action Range:
    Touch Components: V, S, M (diamond dust worth 100+ GP, which the spell consumes) Duration: Concentration,
    up to 1 hour Until the spell ends, one willing creature you touch has Resistance to Bludgeoning, Piercing,
    and Slashing damage.'
  sha256: 023e5b5a47732b661a2f5747da45a9b37648a6e3e18482910c990ee4e99a458b
  status: contract_specified
splat_requires_mechanics: *id001
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
