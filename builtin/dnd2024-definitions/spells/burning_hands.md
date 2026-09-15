---
splat_schema: 1
splat_id: 1ded0ab1-4ede-507e-b5f4-833d13a0d3fc
splat_kind: spell
splat_key: dnd2024:spells/burning_hands
splat_name: Burning Hands
splat_origin:
  key: dnd2024:reference/spell/burning_hands
splat_sources:
- title: SRD 5.2.1
  locator: pp. 114
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: evocation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/burning_hands
  implementation: dnd2024:spell_burning_hands
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    area:
      shape: cone
      length_feet: 15
      origin: caster
    creatures:
      save:
        ability: dexterity
        dc: spell_save_dc
      damage:
        dice:
          base_count: 3
          sides: 6
          extra_per_slot: 1
          base_slot: 1
        type: fire
      success_damage_scale:
        numerator: 1
        denominator: 2
    objects:
      flammable: true
      worn: false
      carried: false
      effect: start_burning
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_burning_hands
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
    objects:
      type: object_list
      required: true
    origin:
      type: location
      required: true
    save_dc:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - areas
  - damage
  - tests
splat_conversion:
  status: specified
  category: spell
  source_sha256: 287957c53eae0d8ce5d6deed11eb4231aec242a95eaaee46c8209f2b49a4fe7b
  source_path: builtin/srd-5.2.1/spell/burning_hands.md
  pages:
  - 114
  engine_gaps: &id001
  - dnd2024:spell_burning_hands
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 7702bdcd27cf16bf607d279a4b4d860197b584eb084bd6ffae1de2ce27cacfe0
splat_clauses:
- key: source_rules
  text: '# Burning Hands Level 1 Evocation (Sorcerer, Wizard) Casting Time: Action Range: Self Components:
    V, S Duration: Instantaneous A thin sheet of flames shoots forth from you. Each creature in a 15-foot
    Cone makes a Dexterity saving throw, taking 3d6 Fire damage on a failed save or half as much damage
    on a successful one. Flammable objects in the Cone that aren’t being worn or carried start burning.
    Using a Higher-Level Spell Slot. The damage increases by 1d6 for each spell slot level above 1.'
  sha256: e3c32850430857bdcac71bd996a01bbaa4d306a2c61b95133f6b469a0fb18708
  status: contract_specified
splat_requires_mechanics: *id001
---

# Burning Hands

<!-- SRD page 114 -->

Level 1 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Instantaneous\
A thin sheet of flames shoots forth from you. Each\
creature in a 15-foot Cone makes a Dexterity saving\
throw, taking 3d6 Fire damage on a failed save or\
half as much damage on a successful one.\
Flammable objects in the Cone that aren’t being\
worn or carried start burning.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/114|page 114]].
