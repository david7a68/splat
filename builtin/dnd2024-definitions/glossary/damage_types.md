---
splat_schema: 1
splat_id: cb0f259c-303d-568d-8bd2-ead92037ef9a
splat_kind: rule
splat_key: dnd2024:glossary/damage_types
splat_name: Damage Types
splat_origin:
  key: dnd2024:reference/glossary/damage_types
splat_sources:
- title: SRD 5.2.1
  locator: pp. 180
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/damage_types
  parameters:
    types:
    - acid
    - bludgeoning
    - cold
    - fire
    - force
    - lightning
    - necrotic
    - piercing
    - poison
    - psychic
    - radiant
    - slashing
    - thunder
    intrinsic_type_rules: false
    gm_new_effect_type:
      chooser: gm
      options:
      - acid
      - bludgeoning
      - cold
      - fire
      - force
      - lightning
      - necrotic
      - piercing
      - poison
      - psychic
      - radiant
      - slashing
      - thunder
    example_sources:
      acid:
      - corrosive_liquid
      - digestive_enzyme
      bludgeoning:
      - blunt_object
      - constriction
      - falling
      cold:
      - freezing_water
      - icy_blast
      fire:
      - flame
      - unbearable_heat
      force:
      - pure_magical_energy
      lightning:
      - electricity
      necrotic:
      - life_draining_energy
      piercing:
      - fang
      - puncturing_object
      poison:
      - toxic_gas
      - venom
      psychic:
      - mind_rending_energy
      radiant:
      - holy_energy
      - searing_radiation
      slashing:
      - claw
      - cutting_object
      thunder:
      - concussive_sound
  implementation: dnd2024:glossary_damage_types
splat_contracts:
- id: dnd2024:glossary_damage_types
  inputs:
    effect:
      type: effect
      required: true
    type:
      type: choice
      required: true
  outputs:
  - damage_type
  events: []
  capabilities:
  - damage
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 79f81c186e266963bed811544403e20dbb743684aae49a6512893a0c730c45f5
  source_path: builtin/srd-5.2.1/glossary/damage_types.md
  pages:
  - 180
  engine_gaps: &id001
  - dnd2024:glossary_damage_types
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 816164010294e4dfee3c3465693069e433018ec086b5a3151f7450221d0aecd7
splat_clauses:
- key: source_rules
  text: '# Damage Types Attacks and other harmful effects deal different types of damage. Damage types
    have no rules of their own, but other rules, such as Resistance, rely on the types. The Damage Types
    table offers examples to help a GM assign a type to a new effect. Damage Types Type Examples Acid
    Corrosive liquids, digestive enzymes Bludgeoning Blunt objects, constriction, falling Cold Freezing
    water, icy blasts Fire Flames, unbearable heat Force Pure magical energy Lightning Electricity Type
    Examples Necrotic Life-draining energy Piercing Fangs, puncturing objects Poison Toxic gas, venom
    Psychic Mind-rending energy Radiant Holy energy, searing radiation Slashing Claws, cutting objects
    Thunder Concussive sound'
  sha256: abd5f6849a041d06933807027429eecdc63126742eb3130df83410e4ea6901e8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Damage Types

<!-- SRD page 180 -->

Attacks and other harmful effects deal different\
types of damage. Damage types have no rules of\
their own, but other rules, such as Resistance, rely\
on the types. The Damage Types table offers exam-\
ples to help a GM assign a type to a new effect.\
Damage Types\
Type Examples\
Acid Corrosive liquids, digestive enzymes\
Bludgeoning Blunt objects, constriction, falling\
Cold Freezing water, icy blasts\
Fire Flames, unbearable heat\
Force Pure magical energy\
Lightning Electricity\
Type Examples\
Necrotic Life-draining energy\
Piercing Fangs, puncturing objects\
Poison Toxic gas, venom\
Psychic Mind-rending energy\
Radiant Holy energy, searing radiation\
Slashing Claws, cutting objects\
Thunder Concussive sound\

Original layout: [[../../srd-5.2.1/pages/180|page 180]].
