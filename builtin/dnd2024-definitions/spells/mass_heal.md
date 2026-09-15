---
splat_schema: 1
splat_id: 8afc8feb-d590-59b8-bf92-05c5842b3fff
splat_kind: spell
splat_key: dnd2024:spells/mass_heal
splat_name: Mass Heal
splat_origin:
  key: dnd2024:reference/spell/mass_heal
splat_sources:
- title: SRD 5.2.1
  locator: pp. 147
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: abjuration
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
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/mass_heal
  implementation: dnd2024:spell_mass_heal
  parameters:
    target:
      kind: creature
      count: any_number
      visible_to_caster: true
    healing:
      pool_maximum: 700
      allocation_chooser: caster
      integer_nonnegative_per_target: true
    healed_targets_remove_conditions:
    - blinded
    - deafened
    - poisoned
splat_contracts:
- id: dnd2024:spell_mass_heal
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - healing_allocation
  - removed_conditions
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - healing
  - conditions
  - movement
  - choices
splat_conversion:
  status: specified
  category: spell
  source_sha256: ededeab0fd209d96768a26c98eef0559f7c0fc198f0c90268386f29102f7c3bf
  source_path: builtin/srd-5.2.1/spell/mass_heal.md
  pages:
  - 147
  engine_gaps: &id001
  - dnd2024:spell_mass_heal
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: a2e63156e680b7ac820ffe1c1b7e2580c263ffe54f4044f88eb10b60f86351ce
splat_clauses:
- key: source_rules
  text: '# Mass Heal Level 9 Abjuration (Cleric) Casting Time: Action Range: 60 feet Components: V, S
    Duration: Instantaneous A flood of healing energy flows from you into creatures around you. You restore
    up to 700 Hit Points, divided as you choose among any number of creatures that you can see within
    range. Creatures healed by this spell also have the Blinded, Deafened, and Poisoned conditions removed
    from them.'
  sha256: b58f783b0cc97b001e32d3f4d0d11f63d36e1e96e0ee0b8c2d03cca86f3b400b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mass Heal

<!-- SRD page 147 -->

Level 9 Abjuration (Cleric)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
A flood of healing energy flows from you into\
creatures around you. You restore up to 700 Hit\
Points, divided as you choose among any number of\
creatures that you can see within range. Creatures\
healed by this spell also have the Blinded, Deafened,\
and Poisoned conditions removed from them.\

Original layout: [[../../srd-5.2.1/pages/147|page 147]].
