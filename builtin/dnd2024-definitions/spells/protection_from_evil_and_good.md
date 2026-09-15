---
splat_schema: 1
splat_id: e59c5b60-ee4d-5fd7-87bd-063b867cf8f7
splat_kind: spell
splat_key: dnd2024:spells/protection_from_evil_and_good
splat_name: Protection from Evil and Good
splat_origin:
  key: dnd2024:reference/spell/protection_from_evil_and_good
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: abjuration
  classes:
  - cleric
  - druid
  - paladin
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a flask of Holy Water worth 25+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/protection_from_evil_and_good
  implementation: dnd2024:spell_protection_from_evil_and_good
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      willing: true
      touched: true
    protection:
      duration: spell_duration
      against_types:
      - aberration
      - celestial
      - elemental
      - fey
      - fiend
      - undead
      attacks_against_target: disadvantage
      prevent:
      - possession
      - charmed_condition
      - frightened_condition
      already_affected:
        advantage_on_new_saves_against:
        - possession
        - charmed
        - frightened
        ends_existing_effect: false
splat_contracts:
- id: dnd2024:spell_protection_from_evil_and_good
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
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - conditions
  - saving_throws
  - roll_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: abe99d8f3682c427691dbcebf7e7c5904c720bed0640ea31bfee419f15285287
  source_path: builtin/srd-5.2.1/spell/protection_from_evil_and_good.md
  pages:
  - 157
  engine_gaps: &id001
  - dnd2024:spell_protection_from_evil_and_good
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: da97485d1387747f30ad6ae50a4706e36d0651c2677a485b0f5ebf790ae8f574
splat_clauses:
- key: source_rules
  text: '# Protection from Evil and Good Level 1 Abjuration (Cleric, Druid, Paladin, Warlock, Wizard)
    Casting Time: Action Range: Touch Components: V, S, M (a flask of Holy Water worth 25+ GP, which the
    spell consumes) Duration: Concentration up to 10 minutes Until the spell ends, one willing creature
    you touch is protected against creatures that are Aberrations, Celestials, Elementals, Fey, Fiends,
    or Undead. The protection grants several benefits. Creatures of those types have Disadvantage on attack
    rolls against the target. The target also can’t be possessed by or gain the Charmed or Frightened
    conditions from them. If the target is already possessed, Charmed, or Frightened by such a creature,
    the target has Advantage on any new saving throw against the relevant effect.'
  sha256: 0fc05e36c50dbe2006810e742d9b11271f0961dbc93a96b52908f5c876232d91
  status: contract_specified
splat_requires_mechanics: *id001
---

# Protection from Evil and Good

<!-- SRD page 157 -->

Level 1 Abjuration (Cleric, Druid, Paladin, Warlock,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a flask of Holy Water worth 25+\
GP, which the spell consumes)\
Duration: Concentration up to 10 minutes\
Until the spell ends, one willing creature you touch\
is protected against creatures that are Aberrations,\
Celestials, Elementals, Fey, Fiends, or Undead.\
The protection grants several benefits. Creatures\
of those types have Disadvantage on attack rolls\
against the target. The target also can’t be pos-\
sessed by or gain the Charmed or Frightened condi-\
tions from them. If the target is already possessed,\
Charmed, or Frightened by such a creature, the tar-\
get has Advantage on any new saving throw against\
the relevant effect.\

Original layout: [[../../srd-5.2.1/pages/157|page 157]].
