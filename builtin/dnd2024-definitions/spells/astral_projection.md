---
splat_schema: 1
splat_id: 8dadd17c-9a0c-5858-b18f-560406795e16
splat_kind: spell
splat_key: dnd2024:spells/astral_projection
splat_name: Astral Projection
splat_origin:
  key: dnd2024:reference/spell/astral_projection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 111
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: necromancy
  classes:
  - cleric
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled
  casting_time: 1 hour
  ritual: false
  range:
    feet: 10
  material:
    description: for each of the spell’s targets, one jacinth worth 1,000+ GP and one silver bar worth
      100+ GP, all of which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/astral_projection
  implementation: dnd2024:spell_astral_projection
  parameters:
    targets:
      kind: creature
      maximum: 8
      within_spell_range: true
      willing: true
      additional_target: caster
    already_astral_plane:
      end_immediately: true
    projection:
      plane: astral
      copy:
      - game_statistics
      - possessions
      - body_appearance
      physical_body:
        condition: unconscious
        needs_food: false
        needs_air: false
        ages: false
      silver_cord:
        visible_length_feet: 1
        origin: between_shoulders
        cut_only_by_explicit_effect: true
        cut_effect: kill_body_and_astral_form
      damage_and_effects_shared_between_forms: false
      astral_plane_exit:
        physical_body_and_possessions: travel_along_cord
        reenter_body: new_plane
    end:
      per_target:
      - body_hp_zero
      - astral_form_hp_zero
      all_targets:
        caster_cost: magic_action
        choice: dismiss
      living_target_cleanup:
      - return_to_body
      - end_suspended_animation
splat_contracts:
- id: dnd2024:spell_astral_projection
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
    trigger:
      required: false
      type: event
  outputs:
  - effects
  - participants
  - items
  events:
  - spell.cast_completed
  - damage.applied
  - effect.ended
  - movement.completed
  - hp.changed
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
  - conditions
  - participant_lifecycle
  - inventory
  - forms
splat_conversion:
  status: specified
  category: spell
  source_sha256: ca248e539b7a6e4ed66c32bf024b5c3d4126794309dbbdd471ab78165fc2c631
  source_path: builtin/srd-5.2.1/spell/astral_projection.md
  pages:
  - 111
  engine_gaps: &id001
  - dnd2024:spell_astral_projection
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: ff40ff37e9641e48b5d0e1c7b20a671885018cc01f3476735cdb6bc0020165a5
splat_clauses:
- key: source_rules
  text: '# Astral Projection Level 9 Necromancy (Cleric, Warlock, Wizard) Casting Time: 1 hour Range:
    10 feet Components: V, S, M (for each of the spell’s targets, one jacinth worth 1,000+ GP and one
    silver bar worth 100+ GP, all of which the spell consumes) Duration: Until dispelled You and up to
    eight willing creatures within range project your astral bodies into the Astral Plane (the spell ends
    instantly if you are already on that plane). Each target’s body is left behind in a state of suspended
    animation; it has the Unconscious condition, doesn’t need food or air, and doesn’t age. A target’s
    astral form resembles its body in almost every way, replicating its game statistics and possessions.
    The principal difference is the addition of a silvery cord that trails from between the shoulder blades
    of the astral form. The cord fades from view after 1 foot. If the cord is cut—which happens only when
    an effect states that it does so—the target’s body and astral form both die. A target’s astral form
    can travel through the Astral Plane. The moment an astral form leaves that plane, the target’s body
    and possessions travel along the silver cord, causing the target to re-enter its body on the new plane.
    Any damage or other effects that apply to an astral form have no effect on the target’s body and vice
    versa. If a target’s body or astral form drops to 0 Hit Points, the spell ends for that target. The
    spell ends for all the targets if you take a Magic action to dismiss it. When the spell ends for a
    target who isn’t dead, the target reappears in its body and exits the state of suspended animation.'
  sha256: ccffc99a639941fabd19f8d09c21c16c0effa9bdf175ab1705dcce866285f145
  status: contract_specified
splat_requires_mechanics: *id001
---

# Astral Projection

<!-- SRD page 111 -->

Level 9 Necromancy (Cleric, Warlock, Wizard)\
Casting Time: 1 hour\
Range: 10 feet\
Components: V, S, M (for each of the spell’s targets,\
one jacinth worth 1,000+ GP and one silver bar worth\
100+ GP, all of which the spell consumes)\
Duration: Until dispelled\
You and up to eight willing creatures within range\
project your astral bodies into the Astral Plane\
(the spell ends instantly if you are already on that\
plane). Each target’s body is left behind in a state of\
suspended animation; it has the Unconscious condi-\
tion, doesn’t need food or air, and doesn’t age.\
A target’s astral form resembles its body in almost\
every way, replicating its game statistics and pos-\
sessions. The principal difference is the addition of\
a silvery cord that trails from between the shoulder\
blades of the astral form. The cord fades from view\
after 1 foot. If the cord is cut—which happens only\
when an effect states that it does so—the target’s\
body and astral form both die.\
A target’s astral form can travel through the As-\
tral Plane. The moment an astral form leaves that\
plane, the target’s body and possessions travel\
along the silver cord, causing the target to re-enter\
its body on the new plane.\
Any damage or other effects that apply to an as-\
tral form have no effect on the target’s body and\
vice versa. If a target’s body or astral form drops to\
0 Hit Points, the spell ends for that target. The spell\
ends for all the targets if you take a Magic action to\
dismiss it.\
When the spell ends for a target who isn’t dead,\
the target reappears in its body and exits the state\
of suspended animation.\

Original layout: [[../../srd-5.2.1/pages/111|page 111]].
