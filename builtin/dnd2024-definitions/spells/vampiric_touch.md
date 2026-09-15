---
splat_schema: 1
splat_id: a7b78458-d9b6-56c5-8c6b-a0d5278f759c
splat_kind: spell
splat_key: dnd2024:spells/vampiric_touch
splat_name: Vampiric Touch
splat_origin:
  key: dnd2024:reference/spell/vampiric_touch
splat_sources:
- title: SRD 5.2.1
  locator: pp. 171
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: necromancy
  classes:
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/vampiric_touch
  implementation: dnd2024:spell_vampiric_touch
  parameters:
    targeting:
      kind: creature
      count: 1
      within_reach: true
    attack:
      kind: melee_spell
      hit:
        damage:
          dice: 3d6
          type: necrotic
        caster_healing:
          fraction_of_necrotic_damage_dealt:
            numerator: 1
            denominator: 2
          rounding: down
    repeat:
      economy: magic_action
      timing: each_caster_turn
      duration: spell_duration
      target: same_or_different
    slot_scaling:
      above: 3
      damage_per_level: 1d6
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_vampiric_touch
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
  - healing_results
  events:
  - spell.cast_completed
  - turn.started
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
  - healing
splat_conversion:
  status: specified
  category: spell
  source_sha256: 2e92a13f221a5c18093f07d2c756df10f68d1f25c79490f1a8a43d6f75890387
  source_path: builtin/srd-5.2.1/spell/vampiric_touch.md
  pages:
  - 171
  engine_gaps: &id001
  - dnd2024:spell_vampiric_touch
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: ae0e3a3172c12cf12335b1e7af7fe891d145c78168cff244eedd9436b5d1c1f0
splat_clauses:
- key: source_rules
  text: '# Vampiric Touch Level 3 Necromancy (Sorcerer, Warlock, Wizard) Casting Time: Action Range: Self
    Components: V, S Duration: Concentration, up to 1 minute The touch of your shadow-wreathed hand can
    siphon life force from others to heal your wounds. Make a melee spell attack against one creature
    within reach. On a hit, the target takes 3d6 Necrotic damage, and you regain Hit Points equal to half
    the amount of Necrotic damage dealt. Until the spell ends, you can make the attack again on each of
    your turns as a Magic action, targeting the same creature or a different one. Using a Higher-Level
    Spell Slot. The damage increases by 1d6 for each spell slot level above 3.'
  sha256: e5eab16b19ee83ffa867538be4b8fa610d301df9eea7d290ecf09a1905f03add
  status: contract_specified
splat_requires_mechanics: *id001
---

# Vampiric Touch

<!-- SRD page 171 -->

Level 3 Necromancy (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 1 minute\
The touch of your shadow-wreathed hand can si-\
phon life force from others to heal your wounds.\
Make a melee spell attack against one creature\
within reach. On a hit, the target takes 3d6 Necrotic\
damage, and you regain Hit Points equal to half the\
amount of Necrotic damage dealt.\
Until the spell ends, you can make the attack again\
on each of your turns as a Magic action, targeting\
the same creature or a different one.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 3.\

Original layout: [[../../srd-5.2.1/pages/171|page 171]].
