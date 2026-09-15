---
splat_schema: 1
splat_id: 33a35a43-1180-5533-92d2-9a887f91fa1c
splat_kind: spell
splat_key: dnd2024:spells/shining_smite
splat_name: Shining Smite
splat_origin:
  key: dnd2024:reference/spell/shining_smite
splat_sources:
- title: SRD 5.2.1
  locator: pp. 162
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - paladin
  components:
  - verbal
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Bonus Action, which you take immediately after hitting a creature with a Melee weapon
    or an Unarmed Strike
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/shining_smite
  implementation: dnd2024:spell_shining_smite
  parameters:
    targeting:
      kind: creature_hit_by_triggering_melee_weapon_or_unarmed_strike
    extra_attack_damage:
      dice: 2d6
      type: radiant
    rider:
      duration: spell_duration
      bright_light_radius_feet: 5
      attacks_against_target: advantage
      invisible_condition_benefits: none
    slot_scaling:
      above: 2
      damage_per_level: 1d6
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_shining_smite
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
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  - attack.hit
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - conditions
splat_conversion:
  status: specified
  category: spell
  source_sha256: 7f4a0afdab524d8f5cddfe47d1ee87dc85162dd80c5cf584f18e61686b534a96
  source_path: builtin/srd-5.2.1/spell/shining_smite.md
  pages:
  - 162
  engine_gaps: &id001
  - dnd2024:spell_shining_smite
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 7cd11957c388502303055de7718ad7073db3816aa539dc42a2cedb17a5690bb1
splat_clauses:
- key: source_rules
  text: '# Shining Smite Level 2 Transmutation (Paladin) Casting Time: Bonus Action, which you take immediately
    after hitting a creature with a Melee weapon or an Unarmed Strike Range: Self Component: V Duration:
    Concentration, up to 1 minute The target hit by the strike takes an extra 2d6 Radiant damage from
    the attack. Until the spell ends, the target sheds Bright Light in a 5-foot radius, attack rolls against
    it have Advantage, and it can’t benefit from the Invisible condition. Using a Higher-Level Spell Slot.
    The damage increases by 1d6 for each spell slot level above 2.'
  sha256: 119bd7b3ea136d91b389b10d8cb002fa7c2d76f2a48b0790024ee0aa5ee7ee4b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shining Smite

<!-- SRD page 162 -->

Level 2 Transmutation (Paladin)\
Casting Time: Bonus Action, which you take immedi-\
ately after hitting a creature with a Melee weapon or\
an Unarmed Strike\
Range: Self\
Component: V\
Duration: Concentration, up to 1 minute\
The target hit by the strike takes an extra 2d6 Radi-\
ant damage from the attack. Until the spell ends, the\
target sheds Bright Light in a 5-foot radius, attack\
rolls against it have Advantage, and it can’t benefit\
from the Invisible condition.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/162|page 162]].
