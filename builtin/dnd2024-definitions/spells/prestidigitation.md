---
splat_schema: 1
splat_id: dd7b2596-90d9-52e3-bd16-cf735492dfe2
splat_kind: spell
splat_key: dnd2024:spells/prestidigitation
splat_name: Prestidigitation
splat_origin:
  key: dnd2024:reference/spell/prestidigitation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 154
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: transmutation
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Up to 1 hour
  casting_time: Action
  ritual: false
  range:
    feet: 10
splat_rule:
  mechanic: dnd2024:spells/prestidigitation
  implementation: dnd2024:spell_prestidigitation
  parameters:
    selection:
      within_spell_range: true
      effect_count: 1
      noninstantaneous_active_maximum_across_castings: 3
    modes:
      sensory_effect:
        duration: instantaneous
        harmless: true
        sensory_effect_choice: true
      fire_play:
        duration: instantaneous
        choices:
        - light
        - snuff
        target_kinds:
        - candle
        - torch
        - small_campfire
      clean_or_soil:
        duration: instantaneous
        choices:
        - clean
        - soil
        object_maximum_cubic_feet: 1
      minor_sensation:
        choices:
        - chill
        - warm
        - flavor
        nonliving_material_maximum_cubic_feet: 1
        duration_hours: 1
      magic_mark:
        choices:
        - color
        - small_mark
        - symbol
        target_kinds:
        - object
        - surface
        duration_hours: 1
      minor_creation:
        choices:
        - nonmagical_trinket
        - illusory_image
        must_fit_in_caster_hand: true
        until: caster_next_turn_end
        trinket:
          damage_possible: false
          monetary_worth: 0
splat_contracts:
- id: dnd2024:spell_prestidigitation
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    choices:
      required: false
      type: choice
    items:
      required: false
      type: item_list
  outputs:
  - effects
  - items
  events:
  - spell.cast_completed
  - movement.completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: c9fbbc8773c1708e2b84e0ac7a111bad0a5b52ff2fb13c72cd2c339df55af208
  source_path: builtin/srd-5.2.1/spell/prestidigitation.md
  pages:
  - 154
  engine_gaps: &id001
  - dnd2024:spell_prestidigitation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 0fc8946355f314987677e25f762c5f3e252ee1b0094f8c265d2a7c6a6bcdc45e
splat_clauses:
- key: source_rules
  text: '# Prestidigitation Transmutation Cantrip (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action
    Range: 10 feet Components: V, S Duration: Up to 1 hour You create a magical effect within range. Choose
    the effect from the options below. If you cast this spell multiple times, you can have up to three
    of its non-instantaneous effects active at a time. Sensory Effect. You create an instantaneous, harmless
    sensory effect, such as a shower of sparks, a puff of wind, faint musical notes, or an odd odor. Fire
    Play. You instantaneously light or snuff out a candle, a torch, or a small campfire. Clean or Soil.
    You instantaneously clean or soil an object no larger than 1 cubic foot. Minor Sensation. You chill,
    warm, or flavor up to 1 cubic foot of nonliving material for 1 hour. Magic Mark. You make a color,
    a small mark, or a symbol appear on an object or a surface for 1 hour. Minor Creation. You create
    a nonmagical trinket or an illusory image that can fit in your hand. It lasts until the end of your
    next turn. A trinket can deal no damage and has no monetary worth.'
  sha256: 240f4ebbefb95026aac133c5549a9055550b4bc3a7a7ec495c8000b16ac3a5ca
  status: contract_specified
splat_requires_mechanics: *id001
---

# Prestidigitation

<!-- SRD page 154 -->

Transmutation Cantrip (Bard, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 10 feet\
Components: V, S\
Duration: Up to 1 hour\
You create a magical effect within range. Choose\
the effect from the options below. If you cast this\
spell multiple times, you can have up to three of its\
non-instantaneous effects active at a time.\
Sensory Effect. You create an instantaneous,\
harmless sensory effect, such as a shower of sparks,\
a puff of wind, faint musical notes, or an odd odor.\
Fire Play. You instantaneously light or snuff out a\
candle, a torch, or a small campfire.\
Clean or Soil. You instantaneously clean or soil an\
object no larger than 1 cubic foot.\
Minor Sensation. You chill, warm, or flavor up to\
1 cubic foot of nonliving material for 1 hour.\
Magic Mark. You make a color, a small mark, or a\
symbol appear on an object or a surface for 1 hour.\
Minor Creation. You create a nonmagical trinket\
or an illusory image that can fit in your hand. It\
lasts until the end of your next turn. A trinket can\
deal no damage and has no monetary worth.\

Original layout: [[../../srd-5.2.1/pages/154|page 154]].
