---
splat_schema: 1
splat_id: c96e58c9-06f4-5457-9c3c-43725c3e0c20
splat_kind: spell
splat_key: dnd2024:spells/animal_shapes
splat_name: Animal Shapes
splat_origin:
  key: dnd2024:reference/spell/animal_shapes
splat_sources:
- title: SRD 5.2.1
  locator: pp. 108
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: transmutation
  classes:
  - druid
  components:
  - verbal
  - somatic
  concentration: false
  duration: 24 hours
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/animal_shapes
  implementation: dnd2024:spell_animal_shapes
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 86400
    targets:
      kind: creature
      willing: true
      visible: true
      range_feet: 30
      count: any
    forms:
      chooser: caster
      choice_per_target: true
      type: beast
      maximum_size: large
      maximum_challenge_rating: 4
    transform_again:
      economy: magic_action
      timing: later_caster_turn
      targets: still_transformed_original_targets
      new_choice_per_target: true
      additional_temporary_hit_points: false
    statistics:
      replace: form_stat_block
      retain:
      - creature_type
      - hit_points
      - hit_point_dice
      - alignment
      - communication
      - intelligence
      - wisdom
      - charisma
    restrictions:
      actions_limited_by_form_anatomy: true
      cast_spells: false
      equipment: melded
      equipment_use: false
    temporary_hit_points:
      amount:
        stat:
          of: first_chosen_form
          key: hit_points
      grant_count_per_target: 1
      remove_remaining_on_spell_end: true
    end_target:
      economy: bonus_action
      chooser: target
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_animal_shapes
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
    forms:
      type: definition_list
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - forms
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - forms
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: f76f947cd08adc470dcff8f2384289b2f6073d7ba257564b30d0d1c7fba345af
  source_path: builtin/srd-5.2.1/spell/animal_shapes.md
  pages:
  - 108
  engine_gaps: &id001
  - dnd2024:spell_animal_shapes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 8fce427a5aec2d8cd7da2224bdcf7e6ba3f9effc380e73bd383f1d96459c5958
splat_clauses:
- key: source_rules
  text: '# Animal Shapes Level 8 Transmutation (Druid) Casting Time: Action Range: 30 feet Components:
    V, S Duration: 24 hours Choose any number of willing creatures that you can see within range. Each
    target shape-shifts into a Large or smaller Beast of your choice that has a Challenge Rating of 4
    or lower. You can choose a different form for each target. On later turns, you can take a Magic action
    to transform the targets again. A target’s game statistics are replaced by the chosen Beast’s statistics,
    but the target retains its creature type; Hit Points; Hit Point Dice; alignment; ability to communicate;
    and Intelligence, Wisdom, and Charisma scores. The target’s actions are limited by the Beast form’s
    anatomy, and it can’t cast spells. The target’s equipment melds into the new form, and the target
    can’t use any of that equipment while in that form. The target gains a number of Temporary Hit Points
    equal to the Hit Points of the first form into which it shape-shifts. These Temporary Hit Points vanish
    if any remain when the spell ends. The transformation lasts for the duration or until the target ends
    it as a Bonus Action.'
  sha256: 266d849d413dfe1518b4942379b8a9ac2490f2519ec52571aeaef7cfde7bd486
  status: contract_specified
splat_requires_mechanics: *id001
---

# Animal Shapes

<!-- SRD page 108 -->

Level 8 Transmutation (Druid)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: 24 hours\
Choose any number of willing creatures that you\
can see within range. Each target shape-shifts into\
a Large or smaller Beast of your choice that has a\
Challenge Rating of 4 or lower. You can choose a dif-\
ferent form for each target. On later turns, you can\
take a Magic action to transform the targets again.\
A target’s game statistics are replaced by the\
chosen Beast’s statistics, but the target retains its\
creature type; Hit Points; Hit Point Dice; alignment;\
ability to communicate; and Intelligence, Wisdom,\
and Charisma scores. The target’s actions are lim-\
ited by the Beast form’s anatomy, and it can’t cast\
spells. The target’s equipment melds into the new\
form, and the target can’t use any of that equipment\
while in that form.\
The target gains a number of Temporary Hit\
Points equal to the Hit Points of the first form into\
which it shape-shifts. These Temporary Hit Points\
vanish if any remain when the spell ends. The trans-\
formation lasts for the duration or until the target\
ends it as a Bonus Action.\

Original layout: [[../../srd-5.2.1/pages/108|page 108]].
