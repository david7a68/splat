---
splat_schema: 1
splat_id: 481e80c6-2f2e-5a32-b487-836f6b1955e4
splat_kind: spell
splat_key: dnd2024:spells/blur
splat_name: Blur
splat_origin:
  key: dnd2024:reference/spell/blur
splat_sources:
- title: SRD 5.2.1
  locator: pp. 114
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: illusion
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/blur
  implementation: dnd2024:spell_blur
  parameters:
    targets:
      kind: self
    incoming_attacks:
      disadvantage: true
      duration: spell_duration
      immune_if_attacker_perceives_caster_using:
      - blindsight
      - truesight
splat_contracts:
- id: dnd2024:spell_blur
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    choices:
      required: false
      type: choice
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - spell.cast_completed
  - attack.roll
  - attack.hit
  capabilities:
  - spellcasting
  - effects
  - roll_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: 711c9c5d0461b6ace5e07649f71006613afbe8d219e227c3e95190d6478c961e
  source_path: builtin/srd-5.2.1/spell/blur.md
  pages:
  - 114
  engine_gaps: &id001
  - dnd2024:spell_blur
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: e1b19e3ad2565d316aa9b25d6b416d7ef134a1e6174935f4c07736fae9a3d5e6
splat_clauses:
- key: source_rules
  text: '# Blur Level 2 Illusion (Sorcerer, Wizard) Casting Time: Action Range: Self Components: V Duration:
    Concentration, up to 1 minute Your body becomes blurred. For the duration, any creature has Disadvantage
    on attack rolls against you. An attacker is immune to this effect if it perceives you with Blindsight
    or Truesight.'
  sha256: df19f42b81fe035c4c8b15cc3e263b9047234d3eb59e64e214e6beadcac2a84c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Blur

<!-- SRD page 114 -->

Level 2 Illusion (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V\
Duration: Concentration, up to 1 minute\
Your body becomes blurred. For the duration, any\
creature has Disadvantage on attack rolls against\
you. An attacker is immune to this effect if it per-\
ceives you with Blindsight or Truesight.\

Original layout: [[../../srd-5.2.1/pages/114|page 114]].
