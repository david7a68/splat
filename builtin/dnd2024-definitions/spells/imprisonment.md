---
splat_schema: 1
splat_id: 62767181-90ee-5077-b4b3-73e185eb0d10
splat_kind: spell
splat_key: dnd2024:spells/imprisonment
splat_name: Imprisonment
splat_origin:
  key: dnd2024:reference/spell/imprisonment
splat_sources:
- title: SRD 5.2.1
  locator: pp. 142
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: abjuration
  classes:
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled
  casting_time: 1 minute
  ritual: false
  range:
    feet: 30
  material:
    description: a statuette of the target worth 5,000+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/imprisonment
  implementation: dnd2024:spell_imprisonment
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: true
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        imprisoned: true
      success:
        unaffected: true
        immunity_to_spell_hours: 24
    imprisoned:
      breathing_required: false
      food_required: false
      drink_required: false
      aging: false
      divination_locate_or_perceive: false
      teleport: false
    mode_choice:
    - burial
    - chaining
    - hedged_prison
    - minimus_containment
    - slumber
    modes:
      burial:
        location: beneath_earth
        container: hollow_force_globe
        size: just_fits_target
        pass_in_or_out: false
      chaining:
        condition: restrained
        anchored: ground
        move_by_any_means: false
      hedged_prison:
        location: demiplane
        teleportation: false
        planar_travel: false
        appearance_choice: prison_form
      minimus_containment:
        height_inches: 1
        container: gemstone_or_similar_object
        container_indestructible: true
        light_passes: true
        anything_else_passes: false
      slumber:
        condition: unconscious
        can_awaken: false
    ending_trigger:
      chooser: caster
      decision_type: observable_action_predicate
      gm_approval_required: true
      gm_constraint:
        high_likelihood_within_years: 10
      chosen_at: cast
    dispel_magic:
      required_slot_level: 9
      eligible_targets:
      - prison
      - casting_component
splat_contracts:
- id: dnd2024:spell_imprisonment
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
  - save
  - immunity
  - prison_effect
  - ending_trigger
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - saving_throws
  - conditions
  - movement
  - perception
  - forms
  - inventory
  - choices
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 9e62770b152170e57c8254a2084d3f6a5db75248a8831b3ad8d3346aedd44ac7
  source_path: builtin/srd-5.2.1/spell/imprisonment.md
  pages:
  - 142
  engine_gaps: &id001
  - dnd2024:spell_imprisonment
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 9246f6d277ec629ea126e0bc74de6bb1f91780c36587668b7d3d3d4327f68008
splat_clauses:
- key: source_rules
  text: '# Imprisonment Level 9 Abjuration (Warlock, Wizard) Casting Time: 1 minute Range: 30 feet Components:
    V, S, M (a statuette of the target worth 5,000+ GP) Duration: Until dispelled You create a magical
    restraint to hold a creature that you can see within range. The target must make a Wisdom saving throw.
    On a successful save, the target is unaffected, and it is immune to this spell for the next 24 hours.
    On a failed save, the target is imprisoned. While imprisoned, the target doesn’t need to breathe,
    eat, or drink, and it doesn’t age. Divination spells can’t locate or perceive the imprisoned target,
    and the target can’t teleport. Until the spell ends, the target is also affected by one of the following
    effects of your choice: Burial. The target is entombed beneath the earth in a hollow globe of magical
    force that is just large enough to contain the target. Nothing can pass into or out of the globe.
    Chaining. Chains firmly rooted in the ground hold the target in place. The target has the Restrained
    condition and can’t be moved by any means. Hedged Prison. The target is trapped in a demiplane that
    is warded against teleportation and planar travel. The demiplane is your choice of a labyrinth, a
    cage, a tower, or the like. Minimus Containment. The target becomes 1 inch tall and is trapped inside
    an indestructible gemstone or a similar object. Light can pass through the gemstone (allowing the
    target to see out and other creatures to see in), but nothing else can pass through by any means.
    Slumber. The target has the Unconscious condition and can’t be awoken. Ending the Spell. When you
    cast the spell, specify a trigger that will end it. The trigger can be as simple or as elaborate as
    you choose, but the GM must agree that it has a high likelihood of happening within the next decade.
    The trigger must be an observable action, such as someone making a particular offering at the temple
    of your god, saving your true love, or defeating a specific monster. A Dispel Magic spell can end
    the spell only if it is cast with a level 9 spell slot, targeting either the prison or the component
    used to create it.'
  sha256: 99f60cd486d8c5b526699250c3db8c2911c50657b95c684bdf94f3baddcd08fb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Imprisonment

<!-- SRD page 142 -->

Level 9 Abjuration (Warlock, Wizard)\
Casting Time: 1 minute\
Range: 30 feet\
Components: V, S, M (a statuette of the target worth\
5,000+ GP)\
Duration: Until dispelled\
You create a magical restraint to hold a creature\
that you can see within range. The target must\
make a Wisdom saving throw. On a successful save,\
the target is unaffected, and it is immune to this\
spell for the next 24 hours. On a failed save, the\
target is imprisoned. While imprisoned, the target\
doesn’t need to breathe, eat, or drink, and it doesn’t\
age. Divination spells can’t locate or perceive the\
imprisoned target, and the target can’t teleport.\
Until the spell ends, the target is also affected by\
one of the following effects of your choice:\
Burial. The target is entombed beneath the earth\
in a hollow globe of magical force that is just large\
enough to contain the target. Nothing can pass\
into or out of the globe.\
Chaining. Chains firmly rooted in the ground hold\
the target in place. The target has the Restrained\
condition and can’t be moved by any means.\
Hedged Prison. The target is trapped in a demi-\
plane that is warded against teleportation and\
planar travel. The demiplane is your choice of a\
labyrinth, a cage, a tower, or the like.\
Minimus Containment. The target becomes 1 inch\
tall and is trapped inside an indestructible gem-\
stone or a similar object. Light can pass through\
the gemstone (allowing the target to see out and\
other creatures to see in), but nothing else can\
pass through by any means.\
Slumber. The target has the Unconscious condition\
and can’t be awoken.\
Ending the Spell. When you cast the spell, specify\
a trigger that will end it. The trigger can be as sim-\
ple or as elaborate as you choose, but the GM must\
agree that it has a high likelihood of happening\
within the next decade. The trigger must be an ob-\
servable action, such as someone making a partic-\
ular offering at the temple of your god, saving your\
true love, or defeating a specific monster.\
A Dispel Magic spell can end the spell only if it is\
cast with a level 9 spell slot, targeting either the\
prison or the component used to create it.\

Original layout: [[../../srd-5.2.1/pages/142|page 142]].
