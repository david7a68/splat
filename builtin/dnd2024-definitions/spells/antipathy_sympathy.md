---
splat_schema: 1
splat_id: eb5016a9-6e24-5df0-8b55-1fde1a9b0d32
splat_kind: spell
splat_key: dnd2024:spells/antipathy_sympathy
splat_name: Antipathy/Sympathy
splat_origin:
  key: dnd2024:reference/spell/antipathy_sympathy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 109, 110
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: enchantment
  classes:
  - bard
  - druid
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 10 days
  casting_time: 1 hour
  ritual: false
  range:
    feet: 60
  material:
    description: a mix of vinegar and honey
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/antipathy_sympathy
  implementation: dnd2024:spell_antipathy_sympathy
  parameters:
    target:
      kind: creature_or_object
      count: 1
      visible_to_caster: false
      maximum_size: huge
    creature_kind_choice:
      chooser: caster
      selection: creature_kind
    aura:
      radius_feet: 120
      event: area.enter
      save:
        ability: wisdom
        dc:
          binding: save_dc
        failure:
          apply_chosen_mode: true
        success:
          effect: none
    modes:
      antipathy:
        condition: frightened
        movement:
          event: turn.current
          direction: away_from_target
          maximize_distance: true
          route: safest
      sympathy:
        condition: charmed
        movement:
          event: turn.current
          direction: toward_target
          maximize_distance: true
          route: safest
        within_feet: 5
        prohibit_willing_departure: true
        additional_save:
          event: damage.applied
          source: enchanted_target
          ability: wisdom
          success: end_effect
    repeat_save:
      event: turn.end
      distance_gt_feet: 120
      ability: wisdom
      success: end_effect
    successful_save_immunity:
      minutes: 1
      scope: this_effect
    mode_choice:
      chooser: caster
      options:
      - antipathy
      - sympathy
      at: cast
splat_contracts:
- id: dnd2024:spell_antipathy_sympathy
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
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
  - aura_effect
  - saves
  - conditions
  - movement_constraints
  - immunity
  events:
  - spell.cast_completed
  - turn.end
  - damage.applied
  - area.enter
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - conditions
  - spatial_areas
  - movement
  - inventory
  - choices
  - time
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 6cba992a274908b04efa346ca9faa79a0c2164aaa1e131a8d375f7b855e56a6e
  source_path: builtin/srd-5.2.1/spell/antipathy_sympathy.md
  pages:
  - 109
  - 110
  engine_gaps: &id001
  - dnd2024:spell_antipathy_sympathy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 8e9f0a2d0a57cf19c205be3d955edf72c96fbbed6cedf8db8cbbe21297dd3f42
splat_clauses:
- key: source_rules
  text: '# Antipathy/Sympathy Level 8 Enchantment (Bard, Druid, Wizard) Casting Time: 1 hour Range: 60
    feet Components: V, S, M (a mix of vinegar and honey) Duration: 10 days As you cast the spell, choose
    whether it creates antipathy or sympathy, and target one creature or object that is Huge or smaller.
    Then specify a kind of creature, such as red dragons, goblins, or vampires. A creature of the chosen
    kind makes a Wisdom saving throw when it comes within 120 feet of the target. Your choice of antipathy
    or sympathy determines what happens to a creature when it fails that save: Antipathy. The creature
    has the Frightened condition. The Frightened creature must use its movement on its turns to get as
    far away as possible from the target, moving by the safest route. Sympathy. The creature has the Charmed
    condition. The Charmed creature must use its movement on its turns to get as close as possible to
    the target, moving by the safest route. If the creature is within 5 feet of the target, the creature
    can’t willingly move away. If the target damages the Charmed creature, that creature can make a Wisdom
    saving throw to end the effect, as described below. Ending the Effect. If the Frightened or Charmed
    creature ends its turn more than 120 feet away from the target, the creature makes a Wisdom saving
    throw. On a successful save, the creature is no longer affected by the target. A creature that successfully
    saves against this effect is immune to it for 1 minute, after which it can be affected again.'
  sha256: 0f12df5c6941c1a7b79a1f258a015eb91598f8497c84b0b9615ae0ed5ef6ef2c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Antipathy/Sympathy

<!-- SRD page 109 -->

Level 8 Enchantment (Bard, Druid, Wizard)\
Casting Time: 1 hour\
Range: 60 feet\
Components: V, S, M (a mix of vinegar and honey)\
Duration: 10 days\
As you cast the spell, choose whether it creates\
antipathy or sympathy, and target one creature or\
object that is Huge or smaller. Then specify a kind of\
creature, such as red dragons, goblins, or vampires.\
A creature of the chosen kind makes a Wisdom\
saving throw when it comes within 120 feet of the\
target. Your choice of antipathy or sympathy deter-\
mines what happens to a creature when it fails that\
save:\
Antipathy. The creature has the Frightened condi-\
tion. The Frightened creature must use its move-\
ment on its turns to get as far away as possible\
from the target, moving by the safest route.\
Sympathy. The creature has the Charmed condi-\
tion. The Charmed creature must use its move-\
ment on its turns to get as close as possible to the\
target, moving by the safest route. If the creature\
is within 5 feet of the target, the creature can’t\
willingly move away. If the target damages the\
Charmed creature, that creature can make a\

<!-- SRD page 110 -->

Wisdom saving throw to end the effect, as de-\
scribed below.\
Ending the Effect. If the Frightened or Charmed\
creature ends its turn more than 120 feet away\
from the target, the creature makes a Wisdom sav-\
ing throw. On a successful save, the creature is no\
longer affected by the target. A creature that suc-\
cessfully saves against this effect is immune to it for\
1 minute, after which it can be affected again.\

Original layout: [[../../srd-5.2.1/pages/109|page 109]], [[../../srd-5.2.1/pages/110|page 110]].
