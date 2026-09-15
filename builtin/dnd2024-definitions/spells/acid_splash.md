---
splat_schema: 1
splat_id: 7a061d25-9ada-50ee-b2b8-5b7ccfbd063c
splat_kind: spell
splat_key: dnd2024:spells/acid_splash
splat_name: Acid Splash
splat_origin:
  key: dnd2024:reference/spell/acid_splash
splat_sources:
- title: SRD 5.2.1
  locator: pp. 107
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
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
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/acid_splash
  implementation: dnd2024:spell_acid_splash
  parameters:
    targeting:
      kind: creatures_in_area
      origin: chosen_point_in_range
      shape: sphere
      radius_feet: 5
    save:
      ability: dexterity
      failure:
        damage:
          dice: 1d6
          type: acid
      success:
        damage: 0
    cantrip_scaling:
      character_level_dice:
        '1': 1d6
        '5': 2d6
        '11': 3d6
        '17': 4d6
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_acid_splash
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
    locations:
      type: location_list
      required: true
    save_dc:
      type: integer
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - damage_results
  - locations
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 7e376f8c6540910d51c201cbafbed950b12f28e755e4671b8488a2e327f70076
  source_path: builtin/srd-5.2.1/spell/acid_splash.md
  pages:
  - 107
  engine_gaps: &id001
  - dnd2024:spell_acid_splash
  conversion_gaps: []
  execution_profiles:
  - dnd2024:spells/acid_splash
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 4e5cd47493709879238d615471365582f381f5564d568b6f0af9681b37dc8aa8
splat_clauses:
- key: source_rules
  text: '# Acid Splash Evocation Cantrip (Sorcerer, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S Duration: Instantaneous You create an acidic bubble at a point within range, where it explodes
    in a 5-foot-radius Sphere. Each creature in that Sphere must succeed on a Dexterity saving throw or
    take 1d6 Acid damage. Cantrip Upgrade. The damage increases by 1d6 when you reach levels 5 (2d6),
    11 (3d6), and 17 (4d6).'
  sha256: 21d69fc9ac0187922ab4a41831ceb7fe9d9e6f45e46a9f90ec1210fb68ba62db
  status: contract_specified
splat_requires_mechanics: *id001
---

# Acid Splash

<!-- SRD page 107 -->

Evocation Cantrip (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
You create an acidic bubble at a point within range,\
where it explodes in a 5-foot-radius Sphere. Each\
creature in that Sphere must succeed on a Dexterity\
saving throw or take 1d6 Acid damage.\
Cantrip Upgrade. The damage increases by 1d6\
when you reach levels 5 (2d6), 11 (3d6), and 17\
(4d6).\

Original layout: [[../../srd-5.2.1/pages/107|page 107]].
