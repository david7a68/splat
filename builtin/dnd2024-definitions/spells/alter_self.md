---
splat_schema: 1
splat_id: e12db40f-cc6c-5bad-8299-f6c5898caf0e
splat_kind: spell
splat_key: dnd2024:spells/alter_self
splat_name: Alter Self
splat_origin:
  key: dnd2024:reference/spell/alter_self
splat_sources:
- title: SRD 5.2.1
  locator: pp. 107
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/alter_self
  implementation: dnd2024:spell_alter_self
  parameters:
    target:
      kind: self
      count: 1
      visible_to_caster: false
    choice:
      chooser: caster
      count: 1
      options:
      - aquatic_adaptation
      - change_appearance
      - natural_weapons
      replace_action: magic
    options:
      aquatic_adaptation:
        underwater_breathing: true
        swim_speed:
          stat:
            of: caster
            key: speed
      change_appearance:
        choice_fields:
        - height
        - weight
        - face
        - voice
        - hair_length
        - color
        - distinguishing_features
        - apparent_species
        statistics_change: false
        same_size_category: true
        same_basic_shape: true
        change_action: magic
      natural_weapons:
        growth_choice:
          claws: slashing
          fangs: piercing
          horns: piercing
          hooves: bludgeoning
        attack: unarmed_strike
        damage_replaces_normal:
          dice:
            count: 1
            sides: 6
            add:
              binding: spellcasting_modifier
        attack_ability: spellcasting
        damage_ability: spellcasting
splat_contracts:
- id: dnd2024:spell_alter_self
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    spellcasting_modifier:
      type: integer
      required: true
    spell_attack_modifier:
      required: false
      type: integer
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
  - form_effect
  - granted_actions
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
  - inventory
  - choices
splat_conversion:
  status: specified
  category: spell
  source_sha256: e1cfa74a0cc3e1134714029b787e31cf0d3fe4d2abf9f4e525187ab6b851e20e
  source_path: builtin/srd-5.2.1/spell/alter_self.md
  pages:
  - 107
  engine_gaps: &id001
  - dnd2024:spell_alter_self
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 5e501a5d087f5e6aa31ee10cff2968fbe4f29988b15c73cb7429548367668df9
splat_clauses:
- key: source_rules
  text: '# Alter Self Level 2 Transmutation (Sorcerer, Wizard) Casting Time: Action Range: Self Components:
    V, S Duration: Concentration, up to 1 hour You alter your physical form. Choose one of the following
    options. Its effects last for the duration, during which you can take a Magic action to replace the
    option you chose with a different one. Aquatic Adaptation. You sprout gills and grow webs between
    your fingers. You can breathe underwater and gain a Swim Speed equal to your Speed. Change Appearance.
    You alter your appearance. You decide what you look like, including your height, weight, facial features,
    sound of your voice, hair length, coloration, and other distinguishing characteristics. You can make
    yourself appear as a member of another species, though none of your statistics change. You can’t appear
    as a creature of a different size, and your basic shape stays the same; if you’re bipedal, you can’t
    use this spell to become quadrupedal, for instance. For the duration, you can take a Magic action
    to change your appearance in this way again. Natural Weapons. You grow claws (Slashing), fangs (Piercing),
    horns (Piercing), or hooves (Bludgeoning). When you use your Unarmed Strike to deal damage with that
    new growth, it deals 1d6 damage of the type in parentheses instead of dealing the normal damage for
    your Unarmed Strike, and you use your spellcasting ability modifier for the attack and damage rolls
    rather than using Strength.'
  sha256: 9c68b31b4a9c28c3e9117d354396d4c4c610084a504d297c9a55befc7bfb6f9e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Alter Self

<!-- SRD page 107 -->

Level 2 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 1 hour\
You alter your physical form. Choose one of the\
following options. Its effects last for the duration,\
during which you can take a Magic action to replace\
the option you chose with a different one.\
Aquatic Adaptation. You sprout gills and grow\
webs between your fingers. You can breathe under-\
water and gain a Swim Speed equal to your Speed.\
Change Appearance. You alter your appear-\
ance. You decide what you look like, including your\
height, weight, facial features, sound of your voice,\
hair length, coloration, and other distinguishing\
characteristics. You can make yourself appear as\
a member of another species, though none of your\
statistics change. You can’t appear as a creature of a\
different size, and your basic shape stays the same;\
if you’re bipedal, you can’t use this spell to become\
quadrupedal, for instance. For the duration, you can\
take a Magic action to change your appearance in\
this way again.\
Natural Weapons. You grow claws (Slashing),\
fangs (Piercing), horns (Piercing), or hooves (Blud-\
geoning). When you use your Unarmed Strike to\
deal damage with that new growth, it deals 1d6\
damage of the type in parentheses instead of deal-\
ing the normal damage for your Unarmed Strike,\
and you use your spellcasting ability modifier\
for the attack and damage rolls rather than using\
Strength.\

Original layout: [[../../srd-5.2.1/pages/107|page 107]].
