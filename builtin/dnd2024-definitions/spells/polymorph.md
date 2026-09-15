---
splat_schema: 1
splat_id: b2368195-f6cb-56fe-8385-2ee5e1d60623
splat_kind: spell
splat_key: dnd2024:spells/polymorph
splat_name: Polymorph
splat_origin:
  key: dnd2024:reference/spell/polymorph
splat_sources:
- title: SRD 5.2.1
  locator: pp. 153, 154
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: transmutation
  classes:
  - bard
  - druid
  - sorcerer
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
    feet: 60
  material:
    description: a caterpillar cocoon
    consumed: false
splat_actions:
  cast:
    name: Cast Polymorph
    activation:
      economy: action
      category: magic
      timing: on_turn
      cost_at: dnd2024:spell_slot_expenditure
    targets:
      chosen:
        kind: creature
        count:
          min: 1
          max: 1
        range:
          feet: 60
        visible: true
    steps:
    - key: transform
      operation: dnd2024:polymorph_2024
      target: chosen
      parameters:
        save_ability: wisdom
        save_dc:
          binding: save_dc
        on_success: no_effect
        form_type: beast
        form_chooser: caster
        form_cr_max: target_cr_else_level
        retained:
        - alignment
        - personality
        - creature_type
        - hit_points
        - hit_dice
        replaced: remaining_game_statistics
        temporary_hp: form_hit_points
        end_early: target_has_no_temporary_hp
        remove_remaining_temporary_hp_on_end: true
        concentration: caster
        maximum_duration:
          value: 1
          unit: hour
        action_limit: form_anatomy
        prohibited:
        - speak
        - cast_spells
        equipment: meld_and_suppress_all_benefits
splat_parameters:
  save_dc:
    type: integer
    minimum: 1
    maximum: 40
    required: true
splat_conversion:
  status: specified
  category: spell
  source_sha256: f94e553c0aba3994712b096d81a47a04f315edd2c481fe1773c298885947ff5f
  source_path: builtin/srd-5.2.1/spell/polymorph.md
  pages:
  - 153
  - 154
  engine_gaps: &id001
  - dnd2024:polymorph_2024
  conversion_gaps: []
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Polymorph Level 4 Transmutation (Bard, Druid, Sorcerer, Wizard) Casting Time: Action Range:
    60 feet Components: V, S, M (a caterpillar cocoon) Duration: Concentration, up to 1 hour You attempt
    to transform a creature that you can see within range into a Beast. The target must succeed on a Wisdom
    saving throw or shape-shift into a Beast form for the duration. That form can be any Beast you choose
    that has a Challenge Rating equal to or less than the target’s (or the target’s level if it doesn’t
    have a Challenge Rating). The target’s game statistics are replaced by the stat block of the chosen
    Beast, but the target retains its alignment, personality, creature type, Hit Points, and Hit Point
    Dice. See the “Animals” section of “Monsters” for a sample of Beast stat blocks. The target gains
    a number of Temporary Hit Points equal to the Hit Points of the Beast form. These Temporary Hit Points
    vanish if any remain when the spell ends. The spell ends early on the target if it has no Temporary
    Hit Points left. The target is limited in the actions it can perform by the anatomy of its new form,
    and it can’t speak or cast spells. The target’s gear melds into the new form. The creature can’t use
    or otherwise benefit from any of that equipment.'
  sha256: f172d2faf097d62c1428bc83ca25984d77e92180a0e0e677e59949f2a2c390f3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Polymorph

<!-- SRD page 153 -->

Level 4 Transmutation (Bard, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a caterpillar cocoon)\
Duration: Concentration, up to 1 hour\
You attempt to transform a creature that you can\
see within range into a Beast. The target must suc-\
ceed on a Wisdom saving throw or shape-shift into\
a Beast form for the duration. That form can be any\
Beast you choose that has a Challenge Rating equal\
to or less than the target’s (or the target’s level if\
it doesn’t have a Challenge Rating). The target’s\
game statistics are replaced by the stat block of the\
chosen Beast, but the target retains its alignment,\
personality, creature type, Hit Points, and Hit Point\
Dice. See the “Animals” section of “Monsters” for a\
sample of Beast stat blocks.\
The target gains a number of Temporary Hit\
Points equal to the Hit Points of the Beast form.\
These Temporary Hit Points vanish if any remain\
when the spell ends. The spell ends early on the tar-\
get if it has no Temporary Hit Points left.\
The target is limited in the actions it can perform\
by the anatomy of its new form, and it can’t speak or\
cast spells.\

<!-- SRD page 154 -->

The target’s gear melds into the new form. The\
creature can’t use or otherwise benefit from any of\
that equipment.\

Original layout: [[../../srd-5.2.1/pages/153|page 153]], [[../../srd-5.2.1/pages/154|page 154]].
