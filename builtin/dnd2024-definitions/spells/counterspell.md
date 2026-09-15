---
splat_schema: 1
splat_id: 028154e6-b9bb-57af-b2a1-5990f575e950
splat_kind: spell
splat_key: dnd2024:spells/counterspell
splat_name: Counterspell
splat_origin:
  key: dnd2024:reference/spell/counterspell
splat_sources:
- title: SRD 5.2.1
  locator: pp. 120
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: abjuration
  classes:
  - sorcerer
  - warlock
  - wizard
  components:
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Reaction, which you take when you see a creature within 60 feet of yourself casting a
    spell with Verbal, Somatic, or Material components
  ritual: false
  range:
    feet: 60
splat_actions:
  cast:
    name: Cast Counterspell
    activation:
      economy: reaction
      category: magic
      timing: dnd2024:spell.cast_pending
      cost_at: dnd2024:spell_slot_expenditure
    targets:
      caster:
        kind: creature
        count:
          min: 1
          max: 1
        range:
          feet: 60
        visible: true
    steps:
    - key: interrupt
      operation: dnd2024:counterspell_2024
      target: caster
      parameters:
        triggering_components_any:
        - verbal
        - somatic
        - material
        save_ability: constitution
        save_dc:
          binding: save_dc
        failure:
          spell_effect: dissipate
          casting_economy: wasted
          spell_slot: not_expended
        success: continue_cast
        bind_to: event.use
        other_costs: follow_own_rule
splat_parameters:
  save_dc:
    type: integer
    minimum: 1
    maximum: 40
    required: true
splat_conversion:
  status: specified
  category: spell
  source_sha256: f92ede73da0dc1d86fdbc5a73a04eb29a298dea7de83a5c67776f2e75d6076ca
  source_path: builtin/srd-5.2.1/spell/counterspell.md
  pages:
  - 120
  engine_gaps: &id001
  - dnd2024:counterspell_2024
  conversion_gaps: []
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Counterspell Level 3 Abjuration (Sorcerer, Warlock, Wizard) Casting Time: Reaction, which you
    take when you see a creature within 60 feet of yourself casting a spell with Verbal, Somatic, or Material
    components Range: 60 feet Components: S Duration: Instantaneous You attempt to interrupt a creature
    in the process of casting a spell. The creature makes a Constitution saving throw. On a failed save,
    the spell dissipates with no effect, and the action, Bonus Action, or Reaction used to cast it is
    wasted. If that spell was cast with a spell slot, the slot isn’t expended.'
  sha256: 4c8266f07c0a21c32690b5e195d3c96a58abad83b10338ff6317a78aa03d6fb9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Counterspell

<!-- SRD page 120 -->

Level 3 Abjuration (Sorcerer, Warlock, Wizard)\
Casting Time: Reaction, which you take when you see a\
creature within 60 feet of yourself casting a spell with\
Verbal, Somatic, or Material components\
Range: 60 feet\
Components: S\
Duration: Instantaneous\
You attempt to interrupt a creature in the process of\
casting a spell. The creature makes a Constitution\
saving throw. On a failed save, the spell dissipates\
with no effect, and the action, Bonus Action, or Re-\
action used to cast it is wasted. If that spell was cast\
with a spell slot, the slot isn’t expended.\

Original layout: [[../../srd-5.2.1/pages/120|page 120]].
