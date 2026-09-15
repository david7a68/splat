---
splat_schema: 1
splat_id: 9473225a-5ed8-52cb-9bad-96498ac5c047
splat_kind: spell
splat_key: dnd2024:spells/true_polymorph
splat_name: True Polymorph
splat_origin:
  key: dnd2024:reference/spell/true_polymorph
splat_sources:
- title: SRD 5.2.1
  locator: pp. 170
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: transmutation
  classes:
  - bard
  - warlock
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
    feet: 30
  material:
    description: a drop of mercury, a dollop of gum arabic, and a wisp of smoke
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/true_polymorph
  implementation: dnd2024:spell_true_polymorph
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 3600
    target:
      count: 1
      visible: true
      range_feet: 30
      kinds:
      - creature
      - nonmagical_object
      object_requirements:
        worn: false
        carried: false
    resistance:
      subject: unwilling_creature
      save:
        ability: wisdom
        dc: spell_save_dc
      success: unaffected
    lifetime:
      initial: concentration_up_to_3600_seconds
      ends_if_target:
      - dies
      - destroyed
      full_concentration_seconds: 3600
      full_concentration_result: until_dispelled
    choice:
      chooser: caster
      modes:
      - creature_into_creature
      - object_into_creature
      - creature_into_object
    creature_into_creature:
      form:
        kind: any_creature
        maximum_challenge_rating: target_challenge_rating_or_level
      statistics:
        replace: new_form_stat_block
        retain:
        - hit_points
        - hit_point_dice
        - alignment
        - personality
      temporary_hit_points:
        amount:
          stat:
            of: new_form
            key: hit_points
        remaining_vanish_on_spell_end: true
      restrictions:
        actions_limited_by_anatomy: true
        speak: false
        cast_spells: false
        gear: melded
        use_equipment: false
        benefit_from_equipment: false
    object_into_creature:
      form:
        kind: any_creature
        maximum_size: object_size
        maximum_challenge_rating: 9
      attitude:
        toward:
        - caster
        - caster_allies
        value: friendly
      combat:
        turn: immediately_after_caster
        obeys_caster_commands: true
      after_elapsed_greater_than_seconds:
        seconds: 3600
        caster_control: ends
        continued_friendly_attitude:
          chooser: gm
          consideration: caster_treatment
      statistics: chosen_creature_stat_block
    creature_into_object:
      form:
        kind: nonmagical_object
        maximum_size: target_creature_size
      transformation_includes:
      - creature
      - worn_items
      - carried_items
      statistics: object_statistics
      after_reversion:
        memory_of_time_in_object_form: none
    target_binding:
      exactly_one:
      - target_creature
      - target_object
      must_match_selected_mode: true
splat_contracts:
- id: dnd2024:spell_true_polymorph
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    mode:
      type: choice
      required: true
    form:
      type: definition
      required: true
    attitude_assessment:
      required: false
      type: choice
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
  - forms
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - forms
  - inventory
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 822f53d4ba493439d07c6102636b4132f9cfab59b56005a74e3fb7c3bb34dcab
  source_path: builtin/srd-5.2.1/spell/true_polymorph.md
  pages:
  - 170
  engine_gaps: &id001
  - dnd2024:spell_true_polymorph
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 847498a5f9c9de8fa19fe00e976438a222f36591a3f7c63fb7e6939d50712b84
splat_clauses:
- key: source_rules
  text: '# True Polymorph Level 9 Transmutation (Bard, Warlock, Wizard) Casting Time: Action Range: 30
    feet Components: V, S, M (a drop of mercury, a dollop of gum arabic, and a wisp of smoke) Duration:
    Concentration, up to 1 hour Choose one creature or nonmagical object that you can see within range.
    The creature shape-shifts into a different creature or a nonmagical object, or the object shape-shifts
    into a creature (the object must be neither worn nor carried). The transformation lasts for the duration
    or until the target dies or is destroyed, but if you maintain Concentration on this spell for the
    full duration, the spell lasts until dispelled. An unwilling creature can make a Wisdom saving throw,
    and if it succeeds, it isn’t affected by this spell. Creature into Creature. If you turn a creature
    into another kind of creature, the new form can be any kind you choose that has a Challenge Rating
    equal to or less than the target’s Challenge Rating or level. The target’s game statistics are replaced
    by the stat block of the new form, but it retains its Hit Points, Hit Point Dice, alignment, and personality.
    The target gains a number of Temporary Hit Points equal to the Hit Points of the new form. These Temporary
    Hit Points vanish if any remain when the spell ends. The target is limited in the actions it can perform
    by the anatomy of its new form, and it can’t speak or cast spells. The target’s gear melds into the
    new form. The creature can’t use or otherwise benefit from any of that equipment. Object into Creature.
    You can turn an object into any kind of creature, as long as the creature’s size is no larger than
    the object’s size and the creature has a Challenge Rating of 9 or lower. The creature is Friendly
    to you and your allies. In combat, it takes its turns immediately after yours, and it obeys your commands.
    If the spell lasts more than an hour, you no longer control the creature. It might remain Friendly
    to you, depending on how you have treated it. Creature into Object. If you turn a creature into an
    object, it transforms along with whatever it is wearing and carrying into that form, as long as the
    object’s size is no larger than the creature’s size. The creature’s statistics become those of the
    object, and the creature has no memory of time spent in this form after the spell ends and it returns
    to normal.'
  sha256: f30c95512983ed14458c6ca4281bb34b8c0e080c0d5c70e766a4805bfbf38a26
  status: contract_specified
splat_requires_mechanics: *id001
---

# True Polymorph

<!-- SRD page 170 -->

Level 9 Transmutation (Bard, Warlock, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a drop of mercury, a dollop of\
gum arabic, and a wisp of smoke)\
Duration: Concentration, up to 1 hour\
Choose one creature or nonmagical object that you\
can see within range. The creature shape-shifts into\
a different creature or a nonmagical object, or the\
object shape-shifts into a creature (the object must\
be neither worn nor carried). The transformation\
lasts for the duration or until the target dies or is\
destroyed, but if you maintain Concentration on\
this spell for the full duration, the spell lasts until\
dispelled.\
An unwilling creature can make a Wisdom saving\
throw, and if it succeeds, it isn’t affected by this spell.\
Creature into Creature. If you turn a creature\
into another kind of creature, the new form can be\
any kind you choose that has a Challenge Rating\
equal to or less than the target’s Challenge Rating\
or level. The target’s game statistics are replaced by\
the stat block of the new form, but it retains its Hit\
Points, Hit Point Dice, alignment, and personality.\
The target gains a number of Temporary Hit\
Points equal to the Hit Points of the new form.\
These Temporary Hit Points vanish if any remain\
when the spell ends.\
The target is limited in the actions it can perform\
by the anatomy of its new form, and it can’t speak or\
cast spells.\
The target’s gear melds into the new form. The\
creature can’t use or otherwise benefit from any of\
that equipment.\
Object into Creature. You can turn an object into\
any kind of creature, as long as the creature’s size\
is no larger than the object’s size and the creature\
has a Challenge Rating of 9 or lower. The creature is\
Friendly to you and your allies. In combat, it takes\
its turns immediately after yours, and it obeys your\
commands.\
If the spell lasts more than an hour, you no longer\
control the creature. It might remain Friendly to\
you, depending on how you have treated it.\
Creature into Object. If you turn a creature into\
an object, it transforms along with whatever it is\
wearing and carrying into that form, as long as the\
object’s size is no larger than the creature’s size. The\
creature’s statistics become those of the object, and\
the creature has no memory of time spent in this\
form after the spell ends and it returns to normal.\

Original layout: [[../../srd-5.2.1/pages/170|page 170]].
