---
splat_schema: 1
splat_id: d96ef08a-2140-5fba-aba1-c9c035989f6f
splat_kind: spell
splat_key: dnd2024:spells/disguise_self
splat_name: Disguise Self
splat_origin:
  key: dnd2024:reference/spell/disguise_self
splat_sources:
- title: SRD 5.2.1
  locator: pp. 124
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: illusion
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/disguise_self
  implementation: dnd2024:spell_disguise_self
  parameters:
    targeting:
      kind: self
      includes:
      - clothing
      - armor
      - weapons
      - carried_belongings
    illusion:
      duration: spell_duration
      height_change_max_feet: 1
      weight_appearance:
      - heavier
      - lighter
      limb_arrangement: same_basic
      remaining_appearance: caster_choice
      physical_inspection: fails
      physical_substance: false
    discern:
      economy: study_action
      check:
        ability: intelligence
        skill: investigation
        dc: spell_save_dc
      success: disguise_discerned
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_disguise_self
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
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
  source_sha256: f9456416d7b86fe409b6a8e31f19742843e356ae4f5520115bc4c04ec6bee5af
  source_path: builtin/srd-5.2.1/spell/disguise_self.md
  pages:
  - 124
  engine_gaps: &id001
  - dnd2024:spell_disguise_self
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 4f17b83a9fe638129be326eb2058786a1a12027b99cf9a075aaa607580c25117
splat_clauses:
- key: source_rules
  text: '# Disguise Self Level 1 Illusion (Bard, Sorcerer, Wizard) Casting Time: Action Range: Self Components:
    V, S Duration: 1 hour You make yourself—including your clothing, armor, weapons, and other belongings
    on your person— look different until the spell ends. You can seem 1 foot shorter or taller and can
    appear heavier or lighter. You must adopt a form that has the same basic arrangement of limbs as you
    have. Otherwise, the extent of the illusion is up to you. The changes wrought by this spell fail to
    hold up to physical inspection. For example, if you use this spell to add a hat to your outfit, objects
    pass through the hat, and anyone who touches it would feel nothing. To discern that you are disguised,
    a creature must take the Study action to inspect your appearance and succeed on an Intelligence (Investigation)
    check against your spell save DC.'
  sha256: 7c1254018ef748011fde5b2b26b3ff4a3f7637eae39eb33cfe4de750216e1ca2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Disguise Self

<!-- SRD page 124 -->

Level 1 Illusion (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: 1 hour\
You make yourself—including your clothing, armor,\
weapons, and other belongings on your person—\
look different until the spell ends. You can seem\
1 foot shorter or taller and can appear heavier or\
lighter. You must adopt a form that has the same\
basic arrangement of limbs as you have. Otherwise,\
the extent of the illusion is up to you.\
The changes wrought by this spell fail to hold\
up to physical inspection. For example, if you use\
this spell to add a hat to your outfit, objects pass\
through the hat, and anyone who touches it would\
feel nothing.\
To discern that you are disguised, a creature must\
take the Study action to inspect your appearance\
and succeed on an Intelligence (Investigation) check\
against your spell save DC.\

Original layout: [[../../srd-5.2.1/pages/124|page 124]].
