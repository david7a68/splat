---
splat_schema: 1
splat_id: 3b72bb44-cf34-5ddb-af7b-c08f7b399fac
splat_kind: spell
splat_key: dnd2024:spells/contingency
splat_name: Contingency
splat_origin:
  key: dnd2024:reference/spell/contingency
splat_sources:
- title: SRD 5.2.1
  locator: pp. 119
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: abjuration
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 10 days
  casting_time: 10 minutes
  ritual: false
  range:
    mode: self
  material:
    description: a gem-encrusted statuette of yourself worth 1,500+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/contingency
  implementation: dnd2024:spell_contingency
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 864000
    contingent_spell:
      chooser: caster
      maximum_level: 5
      castable_by_caster: true
      casting_time: action
      can_target_caster: true
      cast_as_part_of_contingency: true
      pay_spell_slots:
      - contingency
      - contingent_spell
      initial_effect: delayed
    trigger:
      chooser: caster
      timing: cast
      kind: declared_circumstance
      resolution_input: trigger_occurrence
      activate: immediately_after_first_occurrence
      optional_at_activation: false
    activation:
      contingent_targets: caster_only
      ignore_normal_additional_targets: true
      end_contingency: true
    end_conditions:
    - duration_elapsed
    - caster_casts_contingency_again
    - material_component_not_on_caster_person
    maximum_active_per_caster: 1
splat_contracts:
- id: dnd2024:spell_contingency
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    contingent_spell:
      type: definition
      required: true
    contingent_spell_level:
      type: integer
      required: true
    declared_trigger:
      type: choice
      required: true
    component:
      type: item_instance
      required: true
  outputs:
  - effects
  events:
  - declared_trigger.occurred
  - item.holder_changed
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - magic_suppression
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: 958465baf5dff732667b8d0ae4d060349222bff22a713cb580a6ff772df33d9b
  source_path: builtin/srd-5.2.1/spell/contingency.md
  pages:
  - 119
  engine_gaps: &id001
  - dnd2024:spell_contingency
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: b8ad9bc648c07b2e305dd40df56b7ffd5b24855aafdad70107446bf740c6e9c1
splat_clauses:
- key: source_rules
  text: '# Contingency Level 6 Abjuration (Wizard) Casting Time: 10 minutes Range: Self Components: V,
    S, M (a gem-encrusted statuette of yourself worth 1,500+ GP) Duration: 10 days Choose a spell of level
    5 or lower that you can cast, that has a casting time of an action, and that can target you. You cast
    that spell—called the contingent spell—as part of casting Contingency, expending spell slots for both,
    but the contingent spell doesn’t come into effect. Instead, it takes effect when a certain trigger
    occurs. You describe that trigger when you cast the two spells. For example, a Contingency cast with
    Water Breathing might stipulate that Water Breathing comes into effect when you are engulfed in water
    or a similar liquid. The contingent spell takes effect immediately after the trigger occurs for the
    first time, whether or not you want it to, and then Contingency ends. The contingent spell takes effect
    only on you, even if it can normally target others. You can use only one Contingency spell at a time.
    If you cast this spell again, the effect of another Contingency spell on you ends. Also, Contingency
    ends on you if its material component is ever not on your person.'
  sha256: 9d2c76170f577cefbec345c323a0bb6e2c7f585778bc38d2e52112e3a97b7d30
  status: contract_specified
splat_requires_mechanics: *id001
---

# Contingency

<!-- SRD page 119 -->

Level 6 Abjuration (Wizard)\
Casting Time: 10 minutes\
Range: Self\
Components: V, S, M (a gem-encrusted statuette of\
yourself worth 1,500+ GP)\
Duration: 10 days\
Choose a spell of level 5 or lower that you can cast,\
that has a casting time of an action, and that can\
target you. You cast that spell—called the contin-\
gent spell—as part of casting Contingency, expend-\
ing spell slots for both, but the contingent spell\
doesn’t come into effect. Instead, it takes effect\
when a certain trigger occurs. You describe that\
trigger when you cast the two spells. For example,\
a Contingency cast with Water Breathing might stip-\
ulate that Water Breathing comes into effect when\
you are engulfed in water or a similar liquid.\
The contingent spell takes effect immediately af-\
ter the trigger occurs for the first time, whether or\
not you want it to, and then Contingency ends.\
The contingent spell takes effect only on you, even\
if it can normally target others. You can use only\
one Contingency spell at a time. If you cast this spell\
again, the effect of another Contingency spell on you\
ends. Also, Contingency ends on you if its material\
component is ever not on your person.\

Original layout: [[../../srd-5.2.1/pages/119|page 119]].
