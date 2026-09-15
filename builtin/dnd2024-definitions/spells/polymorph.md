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
splat_rule:
  mechanic: dnd2024:spells/polymorph
  implementation: dnd2024:spell_polymorph
  parameters:
    targeting:
      kind: creature
      count: 1
      visible: true
      within_range: true
    save:
      ability: wisdom
      failure: transform
      success: no_effect
    form:
      chooser: caster
      type: beast
      maximum_cr: target_cr_or_level_if_no_cr
      replace_stats: chosen_beast_stat_block
      retain:
      - alignment
      - personality
      - creature_type
      - hit_points
      - hit_point_dice
      duration: spell_duration
    temporary_hp:
      amount: beast_form_hit_points
      remainder_vanishes_on_spell_end: true
      target_spell_ends_when: no_temporary_hp_remaining
    restrictions:
      actions: new_anatomy_limits
      speech: forbidden
      spellcasting: forbidden
    equipment:
      melds_into_form: true
      use: forbidden
      benefits: none
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_polymorph
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
    save_dc:
      type: integer
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - participants
  events:
  - spell.cast_completed
  - health.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - participant_lifecycle
  - forms
splat_conversion:
  status: specified
  category: spell
  source_sha256: f94e553c0aba3994712b096d81a47a04f315edd2c481fe1773c298885947ff5f
  source_path: builtin/srd-5.2.1/spell/polymorph.md
  pages:
  - 153
  - 154
  engine_gaps: &id001
  - dnd2024:spell_polymorph
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 1b05a9608dcb789f9abec1fa6f41d3480275a0e10653ef763f5c832eb2c1cf2c
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
