---
splat_schema: 1
splat_id: d651b2ce-1539-507f-aaf0-009ae6bda075
splat_kind: spell
splat_key: dnd2024:spells/seeming
splat_name: Seeming
splat_origin:
  key: dnd2024:reference/spell/seeming
splat_sources:
- title: SRD 5.2.1
  locator: pp. 160, 161
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: illusion
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 8 hours
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/seeming
  implementation: dnd2024:spell_seeming
  parameters:
    targeting:
      kind: creatures
      chosen: true
      visible: true
      within_range: true
    unwilling:
      save_optional: true
      ability: charisma
      success: unaffected
    illusion:
      appearance_per_target: same_or_different_caster_choice
      includes:
      - body
      - equipment
      height_change_max_feet: 1
      weight_appearance:
      - heavier
      - lighter
      limb_arrangement: same_basic
      remaining_appearance: caster_choice
      duration: spell_duration
      physical_inspection: fails
      physical_substance: false
    discern:
      economy: study_action
      check:
        ability: intelligence
        skill: investigation
        dc: spell_save_dc
      success: observer_knows_disguised
    spell_lifetime:
      concentration: false
      maximum:
        value: 8
        unit: hour
splat_contracts:
- id: dnd2024:spell_seeming
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
    save_dc:
      type: integer
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - perception
splat_conversion:
  status: specified
  category: spell
  source_sha256: d5f2b79072a0cfe5f265885dc44a4c5232cdf52d8686f304912bd34c1705a8c2
  source_path: builtin/srd-5.2.1/spell/seeming.md
  pages:
  - 160
  - 161
  engine_gaps: &id001
  - dnd2024:spell_seeming
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 9f76fac1f46acbfe19f2d8990188bd987d736ad931368af24fa0870967d29318
splat_clauses:
- key: source_rules
  text: '# Seeming Level 5 Illusion (Bard, Sorcerer, Wizard) Casting Time: Action Range: 30 feet Components:
    V, S Duration: 8 hours You give an illusory appearance to each creature of your choice that you can
    see within range. An unwilling target can make a Charisma saving throw, and if it succeeds, it is
    unaffected by this spell. You can give the same appearance or different ones to the targets. The spell
    can change the appearance of the targets’ bodies and equipment. You can make each creature seem 1
    foot shorter or taller and appear heavier or lighter. A target’s new appearance must have the same
    basic arrangement of limbs as the target, but the extent of the illusion is otherwise up to you. The
    spell lasts for the duration. The changes wrought by this spell fail to hold up to physical inspection.
    For example, if you use this spell to add a hat to a creature’s outfit, objects pass through the hat.
    A creature that takes the Study action to examine a target can make an Intelligence (Investigation)
    check against your spell save DC. If it succeeds, it becomes aware that the target is disguised.'
  sha256: be52a10d9d1c2a8654cee82092bd37c5d92d9dad26aa89661ac77b497da516d2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Seeming

<!-- SRD page 160 -->

Level 5 Illusion (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: 8 hours\
You give an illusory appearance to each creature of\
your choice that you can see within range. An un-\
willing target can make a Charisma saving throw,\
and if it succeeds, it is unaffected by this spell.\
You can give the same appearance or different\
ones to the targets. The spell can change the ap-\
pearance of the targets’ bodies and equipment.\
You can make each creature seem 1 foot shorter or\
taller and appear heavier or lighter. A target’s new\
appearance must have the same basic arrangement\
of limbs as the target, but the extent of the illusion\

<!-- SRD page 161 -->

is otherwise up to you. The spell lasts for the\
duration.\
The changes wrought by this spell fail to hold up\
to physical inspection. For example, if you use this\
spell to add a hat to a creature’s outfit, objects pass\
through the hat.\
A creature that takes the Study action to examine\
a target can make an Intelligence (Investigation)\
check against your spell save DC. If it succeeds, it\
becomes aware that the target is disguised.\

Original layout: [[../../srd-5.2.1/pages/160|page 160]], [[../../srd-5.2.1/pages/161|page 161]].
