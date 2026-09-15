---
splat_schema: 1
splat_id: 42b2915c-83c3-55c3-96d8-24755c610813
splat_kind: spell
splat_key: dnd2024:spells/shapechange
splat_name: Shapechange
splat_origin:
  key: dnd2024:reference/spell/shapechange
splat_sources:
- title: SRD 5.2.1
  locator: pp. 161
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: transmutation
  classes:
  - druid
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a jade circlet worth 1,500+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/shapechange
  implementation: dnd2024:spell_shapechange
  parameters:
    target:
      kind: self
      count: 1
      visible_to_caster: false
    form:
      chooser: caster
      maximum_cr:
        source: caster
        stat_choice:
        - level
        - challenge_rating
      sort_previously_seen: true
      excluded_types:
      - construct
      - undead
      change_action: magic
    temporary_hp:
      on_initial_cast_only: true
      amount:
        stat:
          of: initial_form
          key: hit_points
      remaining_vanish_on_end: true
    statistics:
      replace_with: chosen_form_stat_block
      retain:
      - creature_type
      - alignment
      - personality
      - intelligence
      - wisdom
      - charisma
      - hit_points
      - hit_point_dice
      - proficiencies
      - communication_ability
      - spellcasting_feature_if_present
    equipment:
      choice_on_shape_shift:
      - drop_to_ground
      - resize_and_reshape_to_fit
      fit_duration: while_in_form
splat_contracts:
- id: dnd2024:spell_shapechange
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
  - form_effect
  - temporary_hp
  - equipment_transitions
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - healing
  - forms
  - inventory
  - choices
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 82bb0ea32a5b04acb1a2b24324a68856d1e82d55c574ccb4f83261ad7d701136
  source_path: builtin/srd-5.2.1/spell/shapechange.md
  pages:
  - 161
  engine_gaps: &id001
  - dnd2024:spell_shapechange
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 378e64b975c1db487c8b1dbd6cf5da12bff83b83b7bdb7cccdd4fe800605e8ef
splat_clauses:
- key: source_rules
  text: '# Shapechange Level 9 Transmutation (Druid, Wizard) Casting Time: Action Range: Self Components:
    V, S, M (a jade circlet worth 1,500+ GP) Duration: Concentration, up to 1 hour You shape-shift into
    another creature for the duration or until you take a Magic action to shape-shift into a different
    eligible form. The new form must be of a creature that has a Challenge Rating no higher than your
    level or Challenge Rating. You must have seen the sort of creature before, and it can’t be a Construct
    or an Undead. When you cast the spell, you gain a number of Temporary Hit Points equal to the Hit
    Points of the first form into which you shape-shift. These Temporary Hit Points vanish if any remain
    when the spell ends. Your game statistics are replaced by the stat block of the chosen form, but you
    retain your creature type; alignment; personality; Intelligence, Wisdom, and Charisma scores; Hit
    Points; Hit Point Dice; proficiencies; and ability to communicate. If you have the Spellcasting feature,
    you retain it too. Upon shape-shifting, you determine whether your equipment drops to the ground or
    changes in size and shape to fit the new form while you’re in it.'
  sha256: 3536f367a0b82f590470aebb487abeb45f98529f19c6ebcebfef890099b8a0b9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shapechange

<!-- SRD page 161 -->

Level 9 Transmutation (Druid, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a jade circlet worth 1,500+ GP)\
Duration: Concentration, up to 1 hour\
You shape-shift into another creature for the dura-\
tion or until you take a Magic action to shape-shift\
into a different eligible form. The new form must be\
of a creature that has a Challenge Rating no higher\
than your level or Challenge Rating. You must have\
seen the sort of creature before, and it can’t be a\
Construct or an Undead.\
When you cast the spell, you gain a number of\
Temporary Hit Points equal to the Hit Points of the\
first form into which you shape-shift. These Tempo-\
rary Hit Points vanish if any remain when the spell\
ends.\
Your game statistics are replaced by the stat block\
of the chosen form, but you retain your creature\
type; alignment; personality; Intelligence, Wisdom,\
and Charisma scores; Hit Points; Hit Point Dice; pro-\
ficiencies; and ability to communicate. If you have\
the Spellcasting feature, you retain it too.\
Upon shape-shifting, you determine whether your\
equipment drops to the ground or changes in size\
and shape to fit the new form while you’re in it.\

Original layout: [[../../srd-5.2.1/pages/161|page 161]].
