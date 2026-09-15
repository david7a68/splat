---
splat_schema: 1
splat_id: 55c81461-36e7-5f03-97ce-b861b2cdc156
splat_kind: spell
splat_key: dnd2024:spells/greater_restoration
splat_name: Greater Restoration
splat_origin:
  key: dnd2024:reference/spell/greater_restoration
splat_sources:
- title: SRD 5.2.1
  locator: pp. 137
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: abjuration
  classes:
  - bard
  - cleric
  - druid
  - paladin
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: diamond dust worth 100+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/greater_restoration
  implementation: dnd2024:spell_greater_restoration
  parameters:
    targeting:
      kind: creature
      touch: true
      count: 1
    remove_choice_count: 1
    choices:
    - kind: exhaustion
      levels: 1
    - kind: condition
      choices:
      - charmed
      - petrified
    - kind: curse
      include: cursed_item_attunement
    - kind: ability_score_reduction
      ability_count: 1
      reduction: any
    - kind: hp_maximum_reduction
      reduction: any
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_greater_restoration
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
      type: choice
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - conditions
  - health_maximum_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: 97870f1e4c512922694b1885f47dcd7280632c3a4fcdc900a5382f8280c7ed1f
  source_path: builtin/srd-5.2.1/spell/greater_restoration.md
  pages:
  - 137
  engine_gaps: &id001
  - dnd2024:spell_greater_restoration
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: ed1305129ca8e13cf422986993c090da69f43cacbd8480868ece08126e3bcd29
splat_clauses:
- key: source_rules
  text: '# Greater Restoration Level 5 Abjuration (Bard, Cleric, Druid, Paladin, Ranger) Casting Time:
    Action Range: Touch Components: V, S, M (diamond dust worth 100+ GP, which the spell consumes) Duration:
    Instantaneous You touch a creature and magically remove one of the following effects from it: • 1
    Exhaustion level • The Charmed or Petrified condition • A curse, including the target’s Attunement
    to a cursed magic item • Any reduction to one of the target’s ability scores • Any reduction to the
    target’s Hit Point maximum'
  sha256: b7e75e923f34ae355c91950e5f5b27d3877c165e4aa26deb2d589c0e7a0023e4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Greater Restoration

<!-- SRD page 137 -->

Level 5 Abjuration (Bard, Cleric, Druid, Paladin,\
Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (diamond dust worth 100+ GP,\
which the spell consumes)\
Duration: Instantaneous\
You touch a creature and magically remove one of\
the following effects from it:\
•  1 Exhaustion level\
•  The Charmed or Petrified condition\
•  A curse, including the target’s Attunement to a\
cursed magic item\
•  Any reduction to one of the target’s ability scores\
•  Any reduction to the target’s Hit Point maximum\

Original layout: [[../../srd-5.2.1/pages/137|page 137]].
