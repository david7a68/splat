---
splat_schema: 1
splat_id: 1b5855ba-91c4-578c-9f95-eb7745f961ee
splat_kind: spell
splat_key: dnd2024:spells/calm_emotions
splat_name: Calm Emotions
splat_origin:
  key: dnd2024:reference/spell/calm_emotions
splat_sources:
- title: SRD 5.2.1
  locator: pp. 114
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: enchantment
  classes:
  - bard
  - cleric
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/calm_emotions
  implementation: dnd2024:spell_calm_emotions
  parameters:
    targeting:
      kind: creatures_in_area
      types:
      - humanoid
      origin: chosen_point_in_range
      shape: sphere
      radius_feet: 20
    save:
      ability: charisma
      failure_choice_per_target:
      - suppress_conditions
      - suppress_hostility
    suppress_conditions:
      immune:
      - charmed
      - frightened
      suppress_existing:
      - charmed
      - frightened
      duration: spell_duration
    suppress_hostility:
      attitude: indifferent
      toward: caster_chosen_previously_hostile_creatures
      ends_on:
      - target_takes_damage
      - target_witnesses_allies_take_damage
      - spell_ends
      restore_attitude: normal
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_calm_emotions
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
      type: location_list
      required: true
    save_dc:
      type: integer
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - damage_results
  - locations
  events:
  - spell.cast_completed
  - damage.applied
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - conditions
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 1be07b99a6563d8e0b7952ee196a1eefa2dc5b4e01ec813823836a9c1001ea70
  source_path: builtin/srd-5.2.1/spell/calm_emotions.md
  pages:
  - 114
  engine_gaps: &id001
  - dnd2024:spell_calm_emotions
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 05abacea6ed50c409fbf10e035632864a49c34d7b75d1af7ec830c7731645dac
splat_clauses:
- key: source_rules
  text: '# Calm Emotions Level 2 Enchantment (Bard, Cleric) Casting Time: Action Range: 60 feet Components:
    V, S Duration: Concentration, up to 1 minute Each Humanoid in a 20-foot-radius Sphere centered on
    a point you choose within range must succeed on a Charisma saving throw or be affected by one of the
    following effects (choose for each creature): • The creature has Immunity to the Charmed and Frightened
    conditions until the spell ends. If the creature was already Charmed or Frightened, those conditions
    are suppressed for the duration. • The creature becomes Indifferent about creatures of your choice
    that it’s Hostile toward. This indifference ends if the target takes damage or witnesses its allies
    taking damage. When the spell ends, the creature’s attitude returns to normal.'
  sha256: 5076c5aa25836b58bdb67a4b61f1afb14b8941a2df76031d74416641039cb6bf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Calm Emotions

<!-- SRD page 114 -->

Level 2 Enchantment (Bard, Cleric)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
Each Humanoid in a 20-foot-radius Sphere centered\
on a point you choose within range must succeed\
on a Charisma saving throw or be affected by one of\
the following effects (choose for each creature):\
•  The creature has Immunity to the Charmed and\
Frightened conditions until the spell ends. If the\
creature was already Charmed or Frightened,\
those conditions are suppressed for the duration.\
•  The creature becomes Indifferent about crea-\
tures of your choice that it’s Hostile toward. This\
indifference ends if the target takes damage or\
witnesses its allies taking damage. When the spell\
ends, the creature’s attitude returns to normal.\

Original layout: [[../../srd-5.2.1/pages/114|page 114]].
