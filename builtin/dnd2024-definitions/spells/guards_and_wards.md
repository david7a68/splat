---
splat_schema: 1
splat_id: b796317c-1598-54c7-b51e-9b844d81987f
splat_kind: spell
splat_key: dnd2024:spells/guards_and_wards
splat_name: Guards and Wards
splat_origin:
  key: dnd2024:reference/spell/guards_and_wards
splat_sources:
- title: SRD 5.2.1
  locator: pp. 137, 138
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: abjuration
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 24 hours
  casting_time: 1 hour
  ritual: false
  range:
    mode: touch
  material:
    description: a silver rod worth 10+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/guards_and_wards
  implementation: dnd2024:spell_guards_and_wards
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 86400
    area:
      maximum_square_feet: 2500
      maximum_height_feet: 20
      shape_choices:
      - squares: 1
        side_feet: 50
      - squares: 100
        side_feet: 5
        contiguous: true
      - squares: 25
        side_feet: 10
        contiguous: true
    immunity:
      designated_individuals:
        chooser: caster
        timing: cast
      password:
        chooser: caster
        spoken_aloud: true
        within_area_feet: 5
        grant_to: speaker
    dispelling:
      whole_spell_targetable_by_dispel_magic: false
      independently_dispellable_effect_groups:
      - corridors
      - doors
      - stairs
      - other_spell_effect
      all_four_removed: end_spell
    permanence:
      repeat_casts: 365
      frequency: daily
      same_area: true
      result_duration: until_all_effect_groups_dispelled
    corridors:
      scope: all_warded_corridors
      fog_obscurement: heavy
      misdirection:
        at: intersection_or_branch_with_direction_choice
        exclude_caster: true
        probability_percent: 50
        perceived_direction: opposite_chosen_direction
    doors:
      scope: all_warded_doors
      lock_spell:
        key: dnd2024:spells/arcane_lock
      concealed_doors:
        chooser: caster
        maximum: 10
        appearance: plain_wall
    stairs:
      scope: all_warded_stairs_top_to_bottom
      effect_spell:
        key: dnd2024:spells/web
      destroyed_web_regrowth_seconds: 600
      regrowth_requires_spell_active: true
    other_spell_effect:
      choose_count: 1
      chooser: caster
      options:
        dancing_lights:
          spell:
            key: dnd2024:spells/dancing_lights
          locations: 4
          location_kind: corridor
          behavior: repeated_simple_program
          duration: guards_and_wards
        magic_mouth:
          spell:
            key: dnd2024:spells/magic_mouth
          locations: 2
        stinking_cloud:
          spell:
            key: dnd2024:spells/stinking_cloud
          locations: 2
          dispersed_vapor_return_seconds_max: 600
          return_requires_spell_active: true
        gust_of_wind:
          spell:
            key: dnd2024:spells/gust_of_wind
          locations: 1
          location_kinds:
          - corridor
          - room
          wind: continuous_for_duration
        suggestion:
          spell:
            key: dnd2024:spells/suggestion
          square_side_feet: 5
          event: creature.entered_area
          delivery: mental
    nested_spell_invocation:
      additional_casting_actions: false
      additional_spell_slots: false
      concentration_owner: guards_and_wards_lifetime
      parent_exemptions_apply: true
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_guards_and_wards
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    area:
      type: location_list
      required: true
    exempt_creatures:
      type: participant_list
      required: true
    password:
      required: false
      type: text
    spell_options:
      type: choice
      required: true
    campaign:
      type: campaign
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - objects
  events:
  - creature.entered_area
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - illusions
  - world_objects
  - areas
  - campaign_time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 82fa68867dd02b81668bf1009099218c85e649c068ac8ab1b982bf92a2f2eab7
  source_path: builtin/srd-5.2.1/spell/guards_and_wards.md
  pages:
  - 137
  - 138
  engine_gaps: &id001
  - dnd2024:spell_guards_and_wards
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 49727515016da8ba008427f2adab5ae2ea167e826f320921484de565dc910033
splat_clauses:
- key: source_rules
  text: '# Guards and Wards Level 6 Abjuration (Bard, Wizard) Casting Time: 1 hour Range: Touch Components:
    V, S, M (a silver rod worth 10+ GP) Duration: 24 hours You create a ward that protects up to 2,500
    square feet of floor space. The warded area can be up to 20 feet tall, and you shape it as one 50-foot
    square, one hundred 5-foot squares that are contiguous, or twenty-five 10-foot squares that are contiguous.
    When you cast this spell, you can specify individuals that are unaffected by the spell’s effects.
    You can also specify a password that, when spoken aloud within 5 feet of the warded area, makes the
    speaker immune to its effects. The spell creates the effects below within the warded area. Dispel
    Magic has no effect on Guards and Wards itself, but each of the following effects can be dispelled.
    If all four are dispelled, Guards and Wards ends. If you cast the spell every day for 365 days on
    the same area, the spell thereafter lasts until all its effects are dispelled. Corridors. Fog fills
    all the warded corridors, making them Heavily Obscured. In addition, at each intersection or branching
    passage offering a choice of direction, there is a 50 percent chance that a creature other than you
    believes it is going in the opposite direction from the one it chooses. Doors. All doors in the warded
    area are magically locked, as if sealed by the Arcane Lock spell. In addition, you can cover up to
    ten doors with an illusion to make them appear as plain sections of wall. Stairs. Webs fill all stairs
    in the warded area from top to bottom, as in the Web spell. These strands regrow in 10 minutes if
    they are destroyed while Guards and Wards lasts. Other Spell Effect. Place one of the following magical
    effects within the warded area: • Dancing Lights in four corridors, with a simple program that the
    lights repeat as long as Guards and Wards lasts • Magic Mouth in two locations • Stinking Cloud in
    two locations (the vapors return within 10 minutes if dispersed while Guards and Wards lasts) • Gust
    of Wind in one corridor or room (the wind blows continuously while the spell lasts) • Suggestion in
    one 5-foot square; any creature that enters that square receives the suggestion mentally'
  sha256: c17e156b5d4bb961dc923e9d736a553ef4812c3de69a0e61dc86df97904b57e2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Guards and Wards

<!-- SRD page 137 -->

Level 6 Abjuration (Bard, Wizard)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (a silver rod worth 10+ GP)\
Duration: 24 hours\
You create a ward that protects up to 2,500 square\
feet of floor space. The warded area can be up to\
20 feet tall, and you shape it as one 50-foot square,\
one hundred 5-foot squares that are contiguous, or\
twenty-five 10-foot squares that are contiguous.\
When you cast this spell, you can specify individu-\
als that are unaffected by the spell’s effects. You can\
also specify a password that, when spoken aloud\
within 5 feet of the warded area, makes the speaker\
immune to its effects.\

<!-- SRD page 138 -->

The spell creates the effects below within the\
warded area. Dispel Magic has no effect on Guards\
and Wards itself, but each of the following effects\
can be dispelled. If all four are dispelled, Guards and\
Wards ends. If you cast the spell every day for 365\
days on the same area, the spell thereafter lasts un-\
til all its effects are dispelled.\
Corridors. Fog fills all the warded corridors,\
making them Heavily Obscured. In addition, at each\
intersection or branching passage offering a choice\
of direction, there is a 50 percent chance that a\
creature other than you believes it is going in the\
opposite direction from the one it chooses.\
Doors. All doors in the warded area are magically\
locked, as if sealed by the Arcane Lock spell. In addi-\
tion, you can cover up to ten doors with an illusion\
to make them appear as plain sections of wall.\
Stairs. Webs fill all stairs in the warded area from\
top to bottom, as in the Web spell. These strands\
regrow in 10 minutes if they are destroyed while\
Guards and Wards lasts.\
Other Spell Effect. Place one of the following mag-\
ical effects within the warded area:\
•  Dancing Lights in four corridors, with a simple\
program that the lights repeat as long as Guards\
and Wards lasts\
•  Magic Mouth in two locations\
•  Stinking Cloud in two locations (the vapors return\
within 10 minutes if dispersed while Guards and\
Wards lasts)\
•  Gust of Wind in one corridor or room (the wind\
blows continuously while the spell lasts)\
•  Suggestion in one 5-foot square; any creature\
that enters that square receives the suggestion\
mentally\

Original layout: [[../../srd-5.2.1/pages/137|page 137]], [[../../srd-5.2.1/pages/138|page 138]].
