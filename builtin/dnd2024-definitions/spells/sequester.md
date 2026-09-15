---
splat_schema: 1
splat_id: 579b889f-b6c7-53dd-b742-da6eec87361a
splat_kind: spell
splat_key: dnd2024:spells/sequester
splat_name: Sequester
splat_origin:
  key: dnd2024:reference/spell/sequester
splat_sources:
- title: SRD 5.2.1
  locator: pp. 161
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: transmutation
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: gem dust worth 5,000+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/sequester
  implementation: dnd2024:spell_sequester
  parameters:
    spell_lifetime:
      concentration: false
      until: dispelled_or_source_specific_end
    target:
      choices:
      - object
      - willing_creature
      touch: true
    protection:
      condition: invisible
      divination_spell_targeting: forbidden
      magical_detection: forbidden
      magical_remote_viewing: forbidden
    creature_extra:
      state: suspended_animation
      condition: unconscious
      aging: false
      food_required: false
      water_required: false
      air_required: false
    optional_end_trigger:
      chooser: caster
      requirement: occurs_or_visible_within_target_miles
      maximum_miles: 1
      supports_elapsed_time: true
      condition_resolution: supplied_trigger_predicate
    end_on_damage:
      event: damage.applied
      subject: target
    target_binding:
      exactly_one:
      - target_creature
      - target_object
      must_match_selected_mode: true
splat_contracts:
- id: dnd2024:spell_sequester
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    end_trigger:
      required: false
      type: choice
    target_creature:
      required: false
      type: participant
    target_object:
      required: false
      type: object
  outputs:
  - effects
  events:
  - damage.applied
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - magic_suppression
  - conditions
splat_conversion:
  status: specified
  category: spell
  source_sha256: 642425cc6916b9b200a53440371127c3969438b0334467274914c950e382eb36
  source_path: builtin/srd-5.2.1/spell/sequester.md
  pages:
  - 161
  engine_gaps: &id001
  - dnd2024:spell_sequester
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: ac7019267fd71d7ce7f6b0c980be99c624f8f44f480bdb1a4ae3e89786b761b8
splat_clauses:
- key: source_rules
  text: '# Sequester Level 7 Transmutation (Wizard) Casting Time: Action Range: Touch Components: V, S,
    M (gem dust worth 5,000+ GP, which the spell consumes) Duration: Until dispelled With a touch, you
    magically sequester an object or a willing creature. For the duration, the target has the Invisible
    condition and can’t be targeted by Divination spells, detected by magic, or viewed remotely with magic.
    If the target is a creature, it enters a state of suspended animation; it has the Unconscious condition,
    doesn’t age, and doesn’t need food, water, or air. You can set a condition for the spell to end early.
    The condition can be anything you choose, but it must occur or be visible within 1 mile of the target.
    Examples include “after 1,000 years” or “when the tarrasque awakens.” This spell also ends if the
    target takes any damage.'
  sha256: 1f3c7b7af37a726dc101380adc4a7be69e4adaf525fa57f70fe61496b57295c0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sequester

<!-- SRD page 161 -->

Level 7 Transmutation (Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (gem dust worth 5,000+ GP,\
which the spell consumes)\
Duration: Until dispelled\
With a touch, you magically sequester an object or a\
willing creature. For the duration, the target has the\
Invisible condition and can’t be targeted by Divina-\
tion spells, detected by magic, or viewed remotely\
with magic.\
If the target is a creature, it enters a state of sus-\
pended animation; it has the Unconscious condition,\
doesn’t age, and doesn’t need food, water, or air.\
You can set a condition for the spell to end early.\
The condition can be anything you choose, but it\
must occur or be visible within 1 mile of the target.\
Examples include “after 1,000 years” or “when the\
tarrasque awakens.” This spell also ends if the tar-\
get takes any damage.\

Original layout: [[../../srd-5.2.1/pages/161|page 161]].
