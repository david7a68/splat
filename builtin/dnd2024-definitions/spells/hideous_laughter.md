---
splat_schema: 1
splat_id: 0b5651a1-6a64-5e9f-89f9-f603fd79b44a
splat_kind: spell
splat_key: dnd2024:spells/hideous_laughter
splat_name: Hideous Laughter
splat_origin:
  key: dnd2024:reference/spell/hideous_laughter
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: enchantment
  classes:
  - bard
  - warlock
  - wizard
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
    description: a tart and a feather
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/hideous_laughter
  implementation: dnd2024:spell_hideous_laughter
  parameters:
    targets:
      kind: creature
      maximum:
        op: add
        args:
        - 1
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
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        conditions:
        - prone
        - incapacitated
        duration: spell_duration
        cannot_end_own_prone: true
        laugh_uncontrollably_if_capable: true
      success:
        effect: none
    repeat_save:
      ability: wisdom
      events:
      - target_turn_end
      - target_takes_damage
      advantage_if: triggered_by_damage
      success: end_spell
splat_contracts:
- id: dnd2024:spell_hideous_laughter
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
  - turn.end
  - damage.applied
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - conditions
  - saving_throws
  - roll_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: cfbeb7f64f69ee5d32911a12feba3f27ef1c213f0436eb10492f7b9fa66ccbdc
  source_path: builtin/srd-5.2.1/spell/hideous_laughter.md
  pages:
  - 140
  engine_gaps: &id001
  - dnd2024:spell_hideous_laughter
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 10c9dc6c3a95dc7e6fcc53f155372aad5a1aeda254c824c0894dc115f3ced5ee
splat_clauses:
- key: source_rules
  text: '# Hideous Laughter Level 1 Enchantment (Bard, Warlock, Wizard) Casting Time: Action Range: 30
    feet Components: V, S, M (a tart and a feather) Duration: Concentration, up to 1 minute One creature
    of your choice that you can see within range makes a Wisdom saving throw. On a failed save, it has
    the Prone and Incapacitated conditions for the duration. During that time, it laughs uncontrollably
    if it’s capable of laughter, and it can’t end the Prone condition on itself. At the end of each of
    its turns and each time it takes damage, it makes another Wisdom saving throw. The target has Advantage
    on the save if the save is triggered by damage. On a successful save, the spell ends. Using a Higher-Level
    Spell Slot. You can target one additional creature for each spell slot level above 1.'
  sha256: 266f101ed63874359a6e48a04a88ff1275292d3047edcafa86965089531f0248
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hideous Laughter

<!-- SRD page 140 -->

Level 1 Enchantment (Bard, Warlock, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a tart and a feather)\
Duration: Concentration, up to 1 minute\
One creature of your choice that you can see within\
range makes a Wisdom saving throw. On a failed\
save, it has the Prone and Incapacitated conditions\
for the duration. During that time, it laughs uncon-\
trollably if it’s capable of laughter, and it can’t end\
the Prone condition on itself.\
At the end of each of its turns and each time it\
takes damage, it makes another Wisdom saving\
throw. The target has Advantage on the save if the\
save is triggered by damage. On a successful save,\
the spell ends.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/140|page 140]].
