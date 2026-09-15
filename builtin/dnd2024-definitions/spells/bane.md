---
splat_schema: 1
splat_id: 066b3f45-a0d3-5d4a-95e8-a3c07f1070e9
splat_kind: spell
splat_key: dnd2024:spells/bane
splat_name: Bane
splat_origin:
  key: dnd2024:reference/spell/bane
splat_sources:
- title: SRD 5.2.1
  locator: pp. 112
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: enchantment
  classes:
  - bard
  - cleric
  - warlock
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a drop of blood
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/bane
  implementation: dnd2024:spell_bane
  parameters:
    targets:
      kind: creature
      maximum:
        op: add
        args:
        - 3
        - op: multiply
          args:
          - 1
          - op: subtract
            args:
            - binding: cast_level
            - 1
      within_spell_range: true
      visible: true
    save:
      ability: charisma
      dc:
        binding: save_dc
      failure:
        roll_modifier:
          rolls:
          - attack
          - save
          amount:
            dice:
              count: 1
              sides: 4
              add: 0
          operation: subtract
          duration: spell_duration
      success:
        effect: none
splat_contracts:
- id: dnd2024:spell_bane
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
  - dice
  - saving_throws
  - roll_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5a5d1bc53a94666ad78e7fdc6f1e9782d330c5e18ea79177e0bb4fa77dad9075
  source_path: builtin/srd-5.2.1/spell/bane.md
  pages:
  - 112
  engine_gaps: &id001
  - dnd2024:spell_bane
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 7a3f260fe80f9bd8ce8e1e314bedafc7177aaaecb8fe81784639d0cc037f8971
splat_clauses:
- key: source_rules
  text: '# Bane Level 1 Enchantment (Bard, Cleric, Warlock) Casting Time: Action Range: 30 feet Components:
    V, S, M (a drop of blood) Duration: Concentration, up to 1 minute Up to three creatures of your choice
    that you can see within range must each make a Charisma saving throw. Whenever a target that fails
    this save makes an attack roll or a saving throw before the spell ends, the target must subtract 1d4
    from the attack roll or save. Using a Higher-Level Spell Slot. You can target one additional creature
    for each spell slot level above 1.'
  sha256: d4e440b689025ef8dff1eac2ca16e7e04472772aa0a3e9edb6af00a137162122
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bane

<!-- SRD page 112 -->

Level 1 Enchantment (Bard, Cleric, Warlock)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a drop of blood)\
Duration: Concentration, up to 1 minute\
Up to three creatures of your choice that you can\
see within range must each make a Charisma saving\
throw. Whenever a target that fails this save makes\
an attack roll or a saving throw before the spell\
ends, the target must subtract 1d4 from the attack\
roll or save.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/112|page 112]].
