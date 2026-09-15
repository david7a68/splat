---
splat_schema: 1
splat_id: 6eba6cf9-5f77-5c1f-8cd1-b5408e407b95
splat_kind: spell
splat_key: dnd2024:spells/blight
splat_name: Blight
splat_origin:
  key: dnd2024:reference/spell/blight
splat_sources:
- title: SRD 5.2.1
  locator: pp. 113
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: necromancy
  classes:
  - druid
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/blight
  implementation: dnd2024:spell_blight
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    modes:
      creature:
        target:
          kind: creature
          visible: true
          range_feet: 30
        save:
          ability: constitution
          dc: spell_save_dc
          automatic_failure_types:
          - plant
        damage:
          dice:
            base_count: 8
            sides: 8
            extra_per_slot: 1
            base_slot: 4
          type: necrotic
        success_damage_scale:
          numerator: 1
          denominator: 2
      plant:
        target:
          kind: plant_object
          magical: false
          creature: false
          range_feet: 30
        saving_throw: false
        effects:
        - wither
        - die
    target_binding:
      exactly_one:
      - target_creature
      - target_object
      must_match_selected_mode: true
splat_contracts:
- id: dnd2024:spell_blight
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    target_mode:
      type: choice
      required: true
    target_creature:
      required: false
      type: participant
    target_object:
      required: false
      type: object
    save_dc:
      type: integer
      required: true
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - tests
splat_conversion:
  status: specified
  category: spell
  source_sha256: cc1a7a49362f404e26add145df376899aa8f9aeca20f1ecb7f68c22be022d4e5
  source_path: builtin/srd-5.2.1/spell/blight.md
  pages:
  - 113
  engine_gaps: &id001
  - dnd2024:spell_blight
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: d6df4d272b51a924af1782d6207708a82b54d6b1d6283403b2df7e943a5f1ee2
splat_clauses:
- key: source_rules
  text: '# Blight Level 4 Necromancy (Druid, Sorcerer, Warlock, Wizard) Casting Time: Action Range: 30
    feet Components: V, S Duration: Instantaneous A creature that you can see within range makes a Constitution
    saving throw, taking 8d8 Necrotic damage on a failed save or half as much damage on a successful one.
    A Plant creature automatically fails the save. Alternatively, target a nonmagical plant that isn’t
    a creature, such as a tree or shrub. It doesn’t make a save; it simply withers and dies. Using a Higher-Level
    Spell Slot. The damage increases by 1d8 for each spell slot level above 4.'
  sha256: e8f2ab57dd2dbba21e350a4704d65a178e54b110b095408ce61bee3c7b60ed55
  status: contract_specified
splat_requires_mechanics: *id001
---

# Blight

<!-- SRD page 113 -->

Level 4 Necromancy (Druid, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Instantaneous\
A creature that you can see within range makes\
a Constitution saving throw, taking 8d8 Necrotic\
damage on a failed save or half as much damage\
on a successful one. A Plant creature automatically\
fails the save.\
Alternatively, target a nonmagical plant that isn’t\
a creature, such as a tree or shrub. It doesn’t make a\
save; it simply withers and dies.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 4.\

Original layout: [[../../srd-5.2.1/pages/113|page 113]].
