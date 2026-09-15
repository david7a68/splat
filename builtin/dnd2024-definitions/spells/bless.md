---
splat_schema: 1
splat_id: 20599c22-b59a-5f0f-935c-d7703c8a4d69
splat_kind: spell
splat_key: dnd2024:spells/bless
splat_name: Bless
splat_origin:
  key: dnd2024:reference/spell/bless
splat_sources:
- title: SRD 5.2.1
  locator: pp. 113
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: enchantment
  classes:
  - cleric
  - paladin
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
    description: a Holy Symbol worth 5+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/bless
  implementation: dnd2024:spell_bless
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
    roll_modifier:
      rolls:
      - attack
      - save
      amount:
        dice:
          count: 1
          sides: 4
          add: 0
      operation: add
      duration: spell_duration
splat_contracts:
- id: dnd2024:spell_bless
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
  - dice
  - saving_throws
  - roll_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: cf58d9a792520206af06fd890746b380df9516b05216185f55237fa088de51aa
  source_path: builtin/srd-5.2.1/spell/bless.md
  pages:
  - 113
  engine_gaps: &id001
  - dnd2024:spell_bless
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 9fd6e56cef36765d1cbf4e256f6464be22f18e6e10dadf61d59267c37d821457
splat_clauses:
- key: source_rules
  text: '# Bless Level 1 Enchantment (Cleric, Paladin) Casting Time: Action Range: 30 feet Components:
    V, S, M (a Holy Symbol worth 5+ GP) Duration: Concentration, up to 1 minute You bless up to three
    creatures within range. Whenever a target makes an attack roll or a saving throw before the spell
    ends, the target adds 1d4 to the attack roll or save. Using a Higher-Level Spell Slot. You can target
    one additional creature for each spell slot level above 1.'
  sha256: 798be14c2d002a6d093def826ca951075cacd2b25353dfca9135126d6690208e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bless

<!-- SRD page 113 -->

Level 1 Enchantment (Cleric, Paladin)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a Holy Symbol worth 5+ GP)\
Duration: Concentration, up to 1 minute\
You bless up to three creatures within range. When-\
ever a target makes an attack roll or a saving throw\
before the spell ends, the target adds 1d4 to the at-\
tack roll or save.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/113|page 113]].
