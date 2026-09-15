---
splat_schema: 1
splat_id: 0beda0ed-f8cd-515d-9e57-9416c350b41b
splat_kind: spell
splat_key: dnd2024:spells/inflict_wounds
splat_name: Inflict Wounds
splat_origin:
  key: dnd2024:reference/spell/inflict_wounds
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: necromancy
  classes:
  - cleric
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/inflict_wounds
  implementation: dnd2024:spell_inflict_wounds
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      touched: true
    save:
      ability: constitution
      dc:
        binding: save_dc
      failure:
        damage:
          dice:
            count:
              op: add
              args:
              - 2
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 1
            sides: 10
            add: 0
          type: necrotic
      success:
        damage:
          dice:
            count:
              op: add
              args:
              - 2
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 1
            sides: 10
            add: 0
          type: necrotic
        damage_fraction:
          numerator: 1
          denominator: 2
splat_contracts:
- id: dnd2024:spell_inflict_wounds
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
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: c1b84c92fdde772b564dc70768c1ed88caa000242b0c28af403436dbac5253f5
  source_path: builtin/srd-5.2.1/spell/inflict_wounds.md
  pages:
  - 143
  engine_gaps: &id001
  - dnd2024:spell_inflict_wounds
  conversion_gaps: []
  execution_profiles:
  - dnd2024:spells/inflict_wounds
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: d60d9b9578b54b78bbf105a44ddb51cb35613d3e9bab99cbc97c3676daba29db
splat_clauses:
- key: source_rules
  text: '# Inflict Wounds Level 1 Necromancy (Cleric) Casting Time: Action Range: Touch Components: V,
    S Duration: Instantaneous A creature you touch makes a Constitution saving throw, taking 2d10 Necrotic
    damage on a failed save or half as much damage on a successful one. Using a Higher-Level Spell Slot.
    The damage increases by 1d10 for each spell slot level above 1.'
  sha256: 372fc04912f0cc2b99a0d4b72c3882069be7802cb4d70aa66fef8be93053ee20
  status: contract_specified
splat_requires_mechanics: *id001
---

# Inflict Wounds

<!-- SRD page 143 -->

Level 1 Necromancy (Cleric)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous\
A creature you touch makes a Constitution saving\
throw, taking 2d10 Necrotic damage on a failed save\
or half as much damage on a successful one.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d10 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/143|page 143]].
